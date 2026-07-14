use ykcode_core::{Node, NodeKind};

pub(crate) const MIME_KIND: &str = "application/x-ykcode-kind";
pub(crate) const MIME_FALLBACK: &str = "text/plain";

pub(crate) fn kind_from_payload(s: &str) -> NodeKind {
    match s {
        "Section" => NodeKind::Section,
        "Stack" => NodeKind::Stack,
        "Text" => NodeKind::Text,
        "Button" => NodeKind::Button,
        "Image" => NodeKind::Image,
        "Container" => NodeKind::Container,
        "Divider" => NodeKind::Divider,
        "Spacer" => NodeKind::Spacer,
        _ => NodeKind::Container,
    }
}

pub(crate) fn node_with_defaults(kind: NodeKind) -> Node {
    let mut node = Node::new(kind.clone());
    node.content = match kind {
        NodeKind::Text => Some("Add your text".into()),
        NodeKind::Button => Some("Button".into()),
        NodeKind::Image => Some("🖼 Add image".into()),
        _ => None,
    };
    node
}
