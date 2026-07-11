use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Opaque identifier for any node in the document tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub Uuid);

impl NodeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Structural kind of a visual node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeKind {
    Section,
    Stack,
    Text,
    Button,
    Image,
    Divider,
    Spacer,
    Container,
}

impl NodeKind {
    pub fn label(&self) -> &'static str {
        match self {
            NodeKind::Section => "Section",
            NodeKind::Stack => "Stack",
            NodeKind::Text => "Text",
            NodeKind::Button => "Button",
            NodeKind::Image => "Image",
            NodeKind::Divider => "Divider",
            NodeKind::Spacer => "Spacer",
            NodeKind::Container => "Container",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            NodeKind::Section => "⬜",
            NodeKind::Stack => "⊞",
            NodeKind::Text => "T",
            NodeKind::Button => "◉",
            NodeKind::Image => "🖼",
            NodeKind::Divider => "—",
            NodeKind::Spacer => "↕",
            NodeKind::Container => "▭",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Display {
    Flex,
    Grid,
    Block,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Alignment {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    Stretch,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spacing {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Spacing {
    pub fn zero() -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }
    }

    pub fn all(v: f32) -> Self {
        Self {
            top: v,
            right: v,
            bottom: v,
            left: v,
        }
    }
}

impl Default for Spacing {
    fn default() -> Self {
        Self::zero()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Size {
    Auto,
    Fixed(f32),
    Percent(f32),
    Fill,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layout {
    pub display: Display,
    pub direction: FlexDirection,
    pub align_items: Alignment,
    pub justify_content: Alignment,
    pub gap: f32,
    pub padding: Spacing,
    pub margin: Spacing,
    pub width: Size,
    pub height: Size,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            display: Display::Flex,
            direction: FlexDirection::Column,
            align_items: Alignment::Start,
            justify_content: Alignment::Start,
            gap: 0.0,
            padding: Spacing::zero(),
            margin: Spacing::zero(),
            width: Size::Fill,
            height: Size::Auto,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const TRANSPARENT: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const BRAND: Self = Self::rgb(155, 123, 255);
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Border {
    pub width: f32,
    pub color: Color,
    pub radius: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shadow {
    pub x: f32,
    pub y: f32,
    pub blur: f32,
    pub spread: f32,
    pub color: Color,
    pub inset: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Appearance {
    pub background: Option<Color>,
    pub border: Option<Border>,
    pub opacity: f32,
    pub shadows: Vec<Shadow>,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            background: None,
            border: None,
            opacity: 1.0,
            shadows: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Typography {
    pub font_family: String,
    pub font_size: f32,
    pub font_weight: u16,
    pub color: Color,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub align: TextAlign,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            font_family: "Manrope, sans-serif".into(),
            font_size: 16.0,
            font_weight: 400,
            color: Color::BLACK,
            line_height: 1.5,
            letter_spacing: 0.0,
            align: TextAlign::Left,
        }
    }
}

/// A single composable node in the document tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
    pub name: String,
    pub children: Vec<NodeId>,
    pub layout: Layout,
    pub appearance: Appearance,
    pub typography: Option<Typography>,
    pub content: Option<String>,
    pub locked: bool,
    pub visible: bool,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        let name = kind.label().to_string();
        Self {
            id: NodeId::new(),
            kind,
            name,
            children: vec![],
            layout: Layout::default(),
            appearance: Appearance::default(),
            typography: None,
            content: None,
            locked: false,
            visible: true,
        }
    }
}

/// A named page within the site.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub root_node: NodeId,
}

/// The full site document — top-level aggregate root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: Uuid,
    pub name: String,
    pub pages: Vec<Page>,
    pub nodes: HashMap<NodeId, Node>,
    pub active_page_id: Option<Uuid>,
}

impl Document {
    /// Create a new empty document with a single blank home page.
    pub fn new(name: impl Into<String>) -> Self {
        let root_id = NodeId::new();
        let page_id = Uuid::new_v4();

        let root_node = Node {
            id: root_id,
            kind: NodeKind::Section,
            name: "Page root".into(),
            children: vec![],
            layout: Layout {
                padding: Spacing::all(24.0),
                ..Layout::default()
            },
            appearance: Appearance {
                background: Some(Color::WHITE),
                ..Appearance::default()
            },
            typography: None,
            content: None,
            locked: false,
            visible: true,
        };

        let page = Page {
            id: page_id,
            name: "Home".into(),
            slug: "/".into(),
            root_node: root_id,
        };

        let mut nodes = HashMap::new();
        nodes.insert(root_id, root_node);

        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            pages: vec![page],
            nodes,
            active_page_id: Some(page_id),
        }
    }

    pub fn active_page(&self) -> Option<&Page> {
        self.active_page_id
            .and_then(|id| self.pages.iter().find(|p| p.id == id))
    }

    pub fn node(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new("Untitled Site")
    }
}

impl Document {
    /// Insert a node as a child of the active page's root node.
    pub fn insert_node(&mut self, node: Node) -> NodeId {
        let id = node.id;

        let parent_id = self
            .active_page_id
            .and_then(|pid| self.pages.iter().find(|p| p.id == pid))
            .map(|p| p.root_node);

        self.nodes.insert(id, node);

        if let Some(parent_id) = parent_id {
            if let Some(parent) = self.nodes.get_mut(&parent_id) {
                parent.children.push(id);
            }
        }
        id
    }

    /// Insert a node as a child of a specific parent node.
    pub fn insert_node_into(&mut self, node: Node, parent_id: NodeId) -> NodeId {
        let id = node.id;
        self.nodes.insert(id, node);
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(id);
        }
        id
    }

    /// Remove a node from the tree (detaches from parent, removes from map).
    /// Children of the removed node are left as orphans (not recursively removed).
    pub fn remove_node(&mut self, id: NodeId) {
        for node in self.nodes.values_mut() {
            node.children.retain(|&child| child != id);
        }
        self.nodes.remove(&id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_document_has_home_page() {
        let doc = Document::new("MySite");
        assert_eq!(doc.name, "MySite");
        assert_eq!(doc.pages.len(), 1);
        assert_eq!(doc.pages[0].name, "Home");
        assert_eq!(doc.pages[0].slug, "/");
    }

    #[test]
    fn active_page_returns_home() {
        let doc = Document::default();
        let page = doc.active_page().expect("must have active page");
        assert_eq!(page.name, "Home");
    }

    #[test]
    fn insert_node_adds_to_root_children() {
        let mut doc = Document::default();
        let node = Node::new(NodeKind::Button);
        let id = doc.insert_node(node);

        let root_id = doc.active_page().unwrap().root_node;
        assert!(doc.node(&root_id).unwrap().children.contains(&id));
        assert!(doc.node(&id).is_some());
    }

    #[test]
    fn remove_node_detaches_and_deletes() {
        let mut doc = Document::default();
        let node = Node::new(NodeKind::Text);
        let id = doc.insert_node(node);

        let root_id = doc.active_page().unwrap().root_node;
        assert!(doc.node(&root_id).unwrap().children.contains(&id));

        doc.remove_node(id);
        assert!(doc.node(&id).is_none());
        assert!(!doc.node(&root_id).unwrap().children.contains(&id));
    }

    #[test]
    fn insert_multiple_nodes_ordered() {
        let mut doc = Document::default();
        let id_a = doc.insert_node(Node::new(NodeKind::Section));
        let id_b = doc.insert_node(Node::new(NodeKind::Stack));
        let id_c = doc.insert_node(Node::new(NodeKind::Button));

        let root_id = doc.active_page().unwrap().root_node;
        let children = &doc.node(&root_id).unwrap().children;
        assert_eq!(children, &[id_a, id_b, id_c]);
    }

    #[test]
    fn node_defaults_are_visible_and_unlocked() {
        let node = Node::new(NodeKind::Image);
        assert!(node.visible);
        assert!(!node.locked);
        assert!(node.content.is_none());
    }
}
