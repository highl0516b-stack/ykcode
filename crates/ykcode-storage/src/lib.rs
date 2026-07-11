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

/// Fjall-backed persistent store (native platforms only).
#[cfg(feature = "native")]
pub mod native {
    use super::*;
    use fjall::{Config, Keyspace};

    pub struct FjallStore {
        keyspace: Keyspace,
        partition: fjall::PartitionHandle,
    }

    impl FjallStore {
        pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self> {
            let keyspace = Config::new(path).open()?;
            let partition = keyspace.open_partition("documents", Default::default())?;
            Ok(Self {
                keyspace,
                partition,
            })
        }
    }

    impl DocumentStore for FjallStore {
        fn save(&self, doc: &Document) -> Result<(), StorageError> {
            let json = serde_json::to_vec(doc)?;
            self.partition.insert(doc.id.to_string(), json)?;
            self.keyspace.persist(fjall::PersistMode::SyncAll)?;
            Ok(())
        }

        fn load(&self, id: &str) -> Result<Document, StorageError> {
            let bytes = self
                .partition
                .get(id)?
                .ok_or_else(|| StorageError::NotFound(id.to_string()))?;
            let doc = serde_json::from_slice(&bytes)?;
            Ok(doc)
        }

        fn list(&self) -> Result<Vec<String>, StorageError> {
            let ids = self
                .partition
                .iter()
                .filter_map(|item| item.ok())
                .map(|(key, _)| String::from_utf8_lossy(&key).to_string())
                .collect();
            Ok(ids)
        }

        fn delete(&self, id: &str) -> Result<(), StorageError> {
            self.partition.remove(id)?;
            Ok(())
        }
    }
}
