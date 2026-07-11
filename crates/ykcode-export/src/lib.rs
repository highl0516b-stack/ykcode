use std::collections::HashMap;
use ykcode_core::{
    Alignment, Color, Display, Document, FlexDirection, Node, NodeId, NodeKind, Size,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportOutput {
    pub html: String,
    pub css: String,
}

/// Export the active page as standalone HTML + embedded CSS.
pub fn export_document(doc: &Document) -> ExportOutput {
    let page = doc
        .active_page()
        .or_else(|| doc.pages.first())
        .expect("document must have at least one page");

    let root = doc
        .nodes
        .get(&page.root_node)
        .expect("root node must exist");

    let reset = "*,*::before,*::after{box-sizing:border-box}body{margin:0;\
        font-family:Manrope,system-ui,sans-serif}";

    let (body_inner, node_css) = export_node(root, &doc.nodes);
    let css = format!("{reset}{node_css}");

    let html = format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n\
         <meta charset=\"utf-8\"/>\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"/>\n\
         <title>{title}</title>\n\
         <style>{css}</style>\n\
         </head>\n<body>\n{body_inner}\n</body>\n</html>",
        title = escape_html(&doc.name),
        css = css,
        body_inner = body_inner,
    );

    ExportOutput { html, css }
}

/// Recursively render a node into (html_fragment, css_rules).
pub fn export_node(node: &Node, nodes: &HashMap<NodeId, Node>) -> (String, String) {
    if !node.visible {
        return (String::new(), String::new());
    }

    let cls = node_class(node.id);
    let mut css = format!(".{}{{{}}}", cls, layout_to_css(node));

    match node.kind {
        NodeKind::Text => {
            let text = escape_html(node.content.as_deref().unwrap_or("Add your text"));
            let html = format!("<p class=\"{cls}\">{text}</p>");
            (html, css)
        }
        NodeKind::Button => {
            let label = escape_html(node.content.as_deref().unwrap_or("Button"));
            let html = format!("<button type=\"button\" class=\"{cls}\">{label}</button>");
            (html, css)
        }
        NodeKind::Image => {
            let alt = escape_html(node.content.as_deref().unwrap_or("Image"));
            let html = format!("<div class=\"{cls}\" role=\"img\" aria-label=\"{alt}\"></div>");
            (html, css)
        }
        NodeKind::Divider => (format!("<hr class=\"{cls}\"/>"), css),
        NodeKind::Spacer => (
            format!("<div class=\"{cls}\" aria-hidden=\"true\"></div>"),
            css,
        ),
        NodeKind::Section | NodeKind::Stack | NodeKind::Container => {
            let mut children_html = String::new();
            for child_id in &node.children {
                if let Some(child) = nodes.get(child_id) {
                    let (ch, cc) = export_node(child, nodes);
                    children_html.push_str(&ch);
                    css.push_str(&cc);
                }
            }
            let html = format!("<div class=\"{cls}\">{children_html}</div>");
            (html, css)
        }
    }
}

/// Map Layout + Appearance to a CSS declaration string.
pub fn layout_to_css(node: &Node) -> String {
    let mut r: Vec<String> = Vec::new();
    let l = &node.layout;
    let a = &node.appearance;

    match l.display {
        Display::Flex => r.push("display:flex".into()),
        Display::Grid => r.push("display:grid".into()),
        Display::Block => r.push("display:block".into()),
    }

    if matches!(l.display, Display::Flex) {
        r.push(match l.direction {
            FlexDirection::Row => "flex-direction:row".into(),
            FlexDirection::Column => "flex-direction:column".into(),
        });
        r.push(format!("align-items:{}", alignment_css(&l.align_items)));
        r.push(format!(
            "justify-content:{}",
            alignment_css(&l.justify_content)
        ));
    }

    if l.gap > 0.0 {
        r.push(format!("gap:{:.1}px", l.gap));
    }

    let p = &l.padding;
    if p.top > 0.0 || p.right > 0.0 || p.bottom > 0.0 || p.left > 0.0 {
        r.push(format!(
            "padding:{:.1}px {:.1}px {:.1}px {:.1}px",
            p.top, p.right, p.bottom, p.left
        ));
    }

    match &l.width {
        Size::Fixed(v) => r.push(format!("width:{:.1}px", v)),
        Size::Percent(v) => r.push(format!("width:{:.1}%", v)),
        Size::Fill => r.push("width:100%".into()),
        Size::Auto => {}
    }
    match &l.height {
        Size::Fixed(v) => r.push(format!("height:{:.1}px", v)),
        Size::Percent(v) => r.push(format!("height:{:.1}%", v)),
        Size::Fill => r.push("height:100%".into()),
        Size::Auto => {}
    }

    if let Some(bg) = &a.background {
        r.push(format!("background:{}", color_css(bg)));
    }
    if a.opacity < 0.999 {
        r.push(format!("opacity:{:.3}", a.opacity));
    }

    r.join(";")
}

fn alignment_css(a: &Alignment) -> &'static str {
    match a {
        Alignment::Start => "flex-start",
        Alignment::Center => "center",
        Alignment::End => "flex-end",
        Alignment::SpaceBetween => "space-between",
        Alignment::SpaceAround => "space-around",
        Alignment::Stretch => "stretch",
    }
}

fn color_css(c: &Color) -> String {
    format!("rgba({},{},{},{:.3})", c.r, c.g, c.b, c.a as f32 / 255.0)
}

fn node_class(id: NodeId) -> String {
    let s = id.0.to_string().replace('-', "");
    format!("yk-n-{}", &s[..8])
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use ykcode_core::{Node, NodeKind};

    #[test]
    fn export_produces_html_structure() {
        let mut doc = Document::default();
        let mut btn = Node::new(NodeKind::Button);
        btn.content = Some("Click me".into());
        doc.insert_node(btn);
        let out = export_document(&doc);
        assert!(out.html.contains("<!DOCTYPE html>"));
        assert!(out.html.contains("Click me"));
        assert!(out.css.contains("display:flex"));
    }

    #[test]
    fn export_text_renders_paragraph() {
        let mut doc = Document::default();
        let mut t = Node::new(NodeKind::Text);
        t.content = Some("Hello".into());
        doc.insert_node(t);
        let out = export_document(&doc);
        assert!(out.html.contains("<p class=\"yk-n-"));
        assert!(out.html.contains("Hello"));
    }

    #[test]
    fn export_skips_invisible() {
        let mut doc = Document::default();
        let mut n = Node::new(NodeKind::Text);
        n.visible = false;
        n.content = Some("Secret".into());
        doc.insert_node(n);
        let out = export_document(&doc);
        assert!(!out.html.contains("Secret"));
    }
}
