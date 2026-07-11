use anyhow::Result;
use thiserror::Error;
use ykcode_core::Document;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("document not found: {0}")]
    NotFound(String),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("storage backend error: {0}")]
    Backend(#[from] anyhow::Error),
}

/// Trait for persisting and retrieving documents.
pub trait DocumentStore: Send + Sync {
    fn save(&self, doc: &Document) -> Result<(), StorageError>;
    fn load(&self, id: &str) -> Result<Document, StorageError>;
    fn list(&self) -> Result<Vec<String>, StorageError>;
    fn delete(&self, id: &str) -> Result<(), StorageError>;
}

/// In-memory document store for testing and initial dev.
#[derive(Debug, Default)]
pub struct MemoryStore {
    docs: std::sync::Mutex<std::collections::HashMap<String, String>>,
}

impl DocumentStore for MemoryStore {
    fn save(&self, doc: &Document) -> Result<(), StorageError> {
        let json = serde_json::to_string(doc)?;
        let mut guard = self.docs.lock().unwrap();
        guard.insert(doc.id.to_string(), json);
        Ok(())
    }

    fn load(&self, id: &str) -> Result<Document, StorageError> {
        let guard = self.docs.lock().unwrap();
        let json = guard
            .get(id)
            .ok_or_else(|| StorageError::NotFound(id.to_string()))?;
        let doc = serde_json::from_str(json)?;
        Ok(doc)
    }

    fn list(&self) -> Result<Vec<String>, StorageError> {
        let guard = self.docs.lock().unwrap();
        Ok(guard.keys().cloned().collect())
    }

    fn delete(&self, id: &str) -> Result<(), StorageError> {
        let mut guard = self.docs.lock().unwrap();
        guard.remove(id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ykcode_core::Document;

    #[test]
    fn memory_store_save_and_load() {
        let store = MemoryStore::default();
        let doc = Document::new("Test Doc");
        let id = doc.id.to_string();

        store.save(&doc).unwrap();
        let loaded = store.load(&id).unwrap();
        assert_eq!(loaded.name, "Test Doc");
        assert_eq!(loaded.id, doc.id);
    }

    #[test]
    fn memory_store_list_all() {
        let store = MemoryStore::default();
        store.save(&Document::new("A")).unwrap();
        store.save(&Document::new("B")).unwrap();
        let ids = store.list().unwrap();
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn memory_store_delete_removes_entry() {
        let store = MemoryStore::default();
        let doc = Document::new("Temp");
        let id = doc.id.to_string();
        store.save(&doc).unwrap();
        store.delete(&id).unwrap();
        assert!(matches!(store.load(&id), Err(StorageError::NotFound(_))));
    }

    #[test]
    fn memory_store_not_found_error() {
        let store = MemoryStore::default();
        assert!(matches!(store.load("nope"), Err(StorageError::NotFound(_))));
    }
}

/// Fjall-backed persistent store (native platforms only).
#[cfg(feature = "native")]
pub mod native {
    use std::path::Path;

    use super::{Document, DocumentStore, StorageError};
    use anyhow::Result;
    use fjall::{Database, Keyspace, KeyspaceCreateOptions, PersistMode};

    const KEYSPACE_NAME: &str = "documents";

    pub struct FjallStore {
        db: Database,
        docs: Keyspace,
    }

    impl FjallStore {
        pub fn open(path: impl AsRef<Path>) -> Result<Self> {
            let db = Database::builder(path).open()?;
            let docs = db.keyspace(KEYSPACE_NAME, KeyspaceCreateOptions::default)?;
            Ok(Self { db, docs })
        }
    }

    impl DocumentStore for FjallStore {
        fn save(&self, doc: &Document) -> Result<(), StorageError> {
            let bytes = serde_json::to_vec(doc)?;
            self.docs
                .insert(doc.id.to_string(), bytes)
                .map_err(|e| StorageError::Backend(e.into()))?;
            self.db
                .persist(PersistMode::SyncAll)
                .map_err(|e| StorageError::Backend(e.into()))?;
            Ok(())
        }

        fn load(&self, id: &str) -> Result<Document, StorageError> {
            let bytes = self
                .docs
                .get(id)
                .map_err(|e| StorageError::Backend(e.into()))?
                .ok_or_else(|| StorageError::NotFound(id.to_string()))?;
            Ok(serde_json::from_slice(&bytes)?)
        }

        fn list(&self) -> Result<Vec<String>, StorageError> {
            let ids = self
                .docs
                .iter()
                .filter_map(|g| g.key().ok())
                .map(|k| String::from_utf8_lossy(&k).into_owned())
                .collect();
            Ok(ids)
        }

        fn delete(&self, id: &str) -> Result<(), StorageError> {
            self.docs
                .remove(id)
                .map_err(|e| StorageError::Backend(e.into()))?;
            self.db
                .persist(PersistMode::SyncAll)
                .map_err(|e| StorageError::Backend(e.into()))?;
            Ok(())
        }
    }
}
