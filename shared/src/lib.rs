use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;
use uuid::Uuid;

// ── Primitive identifiers ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentId(pub Uuid);

impl ComponentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ComponentId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ComponentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectId(pub Uuid);

impl ProjectId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ProjectId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ProjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ── Component type catalogue ──────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentKind {
    // Basics
    Button,
    Text,
    Image,
    Icon,
    Divider,
    // Layout
    Container,
    Row,
    Column,
    Grid,
    Stack,
    // Navigation
    Navbar,
    Sidebar,
    Tab,
    Breadcrumb,
    // Forms
    Input,
    Textarea,
    Select,
    Checkbox,
    Radio,
    Toggle,
    Slider,
    DatePicker,
    // Media
    Video,
    Audio,
    Carousel,
    // Data display
    Table,
    List,
    Card,
    Badge,
    Tag,
    // Commerce
    ProductCard,
    PriceTag,
    CartIcon,
    // Custom (user-defined reusable)
    Custom { name: String },
}

// ── Design tokens ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum DesignToken {
    Color(String),
    Spacing(f32),
    FontSize(f32),
    FontWeight(u16),
    BorderRadius(f32),
    Shadow(String),
    Duration(u32),
}

// ── Component properties ──────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Bounds {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleProperties {
    pub background: Option<String>,
    pub border_color: Option<String>,
    pub border_width: Option<f32>,
    pub border_radius: Option<f32>,
    pub padding: [f32; 4],
    pub margin: [f32; 4],
    pub opacity: f32,
    pub font_size: Option<f32>,
    pub font_weight: Option<u16>,
    pub color: Option<String>,
    pub text_align: Option<String>,
    pub custom: HashMap<String, String>,
}

impl Default for StyleProperties {
    fn default() -> Self {
        Self {
            background: None,
            border_color: None,
            border_width: None,
            border_radius: None,
            padding: [0.0; 4],
            margin: [0.0; 4],
            opacity: 1.0,
            font_size: None,
            font_weight: None,
            color: None,
            text_align: None,
            custom: HashMap::new(),
        }
    }
}

// ── Canvas component tree node ────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanvasComponent {
    pub id: ComponentId,
    pub kind: ComponentKind,
    pub name: String,
    pub bounds: Bounds,
    pub style: StyleProperties,
    pub children: Vec<CanvasComponent>,
    pub visible: bool,
    pub locked: bool,
    pub rotation: f32,
}

impl CanvasComponent {
    pub fn new(kind: ComponentKind, name: impl Into<String>, bounds: Bounds) -> Self {
        Self {
            id: ComponentId::new(),
            kind,
            name: name.into(),
            bounds,
            style: StyleProperties::default(),
            children: Vec::new(),
            visible: true,
            locked: false,
            rotation: 0.0,
        }
    }
}

// ── Project palette (color-centric approach) ──────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectPalette {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub surface: String,
    pub background: String,
    pub text: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub custom: Vec<(String, String)>,
}

impl Default for ProjectPalette {
    fn default() -> Self {
        Self {
            primary: "#7c5cff".into(),
            secondary: "#35cfff".into(),
            accent: "#f06fff".into(),
            surface: "#141821".into(),
            background: "#090b10".into(),
            text: "#f4f6fc".into(),
            success: "#3cdda4".into(),
            warning: "#ffc45f".into(),
            error: "#ff647c".into(),
            custom: Vec::new(),
        }
    }
}

// ── Page / Artboard ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Artboard {
    pub id: Uuid,
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub components: Vec<CanvasComponent>,
}

impl Artboard {
    pub fn new(name: impl Into<String>, width: f32, height: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            width,
            height,
            components: Vec::new(),
        }
    }

    pub fn mobile(name: impl Into<String>) -> Self {
        Self::new(name, 390.0, 844.0)
    }

    pub fn desktop(name: impl Into<String>) -> Self {
        Self::new(name, 1440.0, 900.0)
    }
}

// ── Project ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub description: String,
    pub palette: ProjectPalette,
    pub artboards: Vec<Artboard>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        let mut artboard = Artboard::mobile("Mobile");
        let button = CanvasComponent::new(
            ComponentKind::Button,
            "Primary Button",
            Bounds::new(100.0, 200.0, 160.0, 48.0),
        );
        artboard.components.push(button);

        Self {
            id: ProjectId::new(),
            name: name.into(),
            description: String::new(),
            palette: ProjectPalette::default(),
            artboards: vec![artboard],
            created_at: now,
            updated_at: now,
        }
    }
}

// ── Domain errors ─────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum YkError {
    #[error("Component not found: {0}")]
    ComponentNotFound(ComponentId),

    #[error("Project not found: {0}")]
    ProjectNotFound(ProjectId),

    #[error("Invalid placement: {reason}")]
    InvalidPlacement { reason: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Storage error: {0}")]
    Storage(String),
}

// ── Progressive disclosure level ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DisclosureLevel {
    #[default]
    Guided,
    Standard,
    Expert,
}

// ── Viewport transform (canvas pan/zoom) ──────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewportTransform {
    pub translate_x: f32,
    pub translate_y: f32,
    pub scale: f32,
}

impl Default for ViewportTransform {
    fn default() -> Self {
        Self {
            translate_x: 0.0,
            translate_y: 0.0,
            scale: 1.0,
        }
    }
}

impl ViewportTransform {
    pub const MIN_SCALE: f32 = 0.25;
    pub const MAX_SCALE: f32 = 20.0;

    pub fn zoom_around(&mut self, delta: f32, cx: f32, cy: f32) {
        let new_scale = (self.scale * (1.0 + delta)).clamp(Self::MIN_SCALE, Self::MAX_SCALE);
        let ratio = new_scale / self.scale;
        self.translate_x = cx - ratio * (cx - self.translate_x);
        self.translate_y = cy - ratio * (cy - self.translate_y);
        self.scale = new_scale;
    }
}

// ── Gesture state ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum GesturePhase {
    #[default]
    Idle,
    DragPending {
        origin_x: f32,
        origin_y: f32,
    },
    Dragging {
        current_x: f32,
        current_y: f32,
    },
    Pinching {
        scale_delta: f32,
        center_x: f32,
        center_y: f32,
    },
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_zoom_around_clamps_to_min() {
        let mut t = ViewportTransform::default();
        // Zoom out far beyond min
        for _ in 0..30 {
            t.zoom_around(-0.5, 0.0, 0.0);
        }
        assert!(
            t.scale >= ViewportTransform::MIN_SCALE,
            "scale {} below minimum {}",
            t.scale,
            ViewportTransform::MIN_SCALE
        );
    }

    #[test]
    fn viewport_zoom_around_clamps_to_max() {
        let mut t = ViewportTransform::default();
        for _ in 0..30 {
            t.zoom_around(0.5, 0.0, 0.0);
        }
        assert!(
            t.scale <= ViewportTransform::MAX_SCALE,
            "scale {} above maximum {}",
            t.scale,
            ViewportTransform::MAX_SCALE
        );
    }

    #[test]
    fn viewport_zoom_around_adjusts_translation() {
        let mut t = ViewportTransform {
            translate_x: 100.0,
            translate_y: 100.0,
            scale: 1.0,
        };
        // Zoom 2× around canvas centre (200, 200)
        t.zoom_around(1.0, 200.0, 200.0);
        assert!(
            (t.scale - 2.0).abs() < 1e-4,
            "expected scale ~2.0, got {}",
            t.scale
        );
        // After 2× zoom around (200,200), translate should shift towards origin
        assert_ne!(t.translate_x, 100.0, "translate_x should change on zoom");
    }

    #[test]
    fn project_new_seeds_artboard_and_button() {
        let p = Project::new("Test");
        assert_eq!(p.name, "Test");
        assert_eq!(p.artboards.len(), 1);
        let ab = &p.artboards[0];
        assert_eq!(ab.name, "Mobile");
        assert_eq!(ab.components.len(), 1);
        assert!(matches!(ab.components[0].kind, ComponentKind::Button));
    }

    #[test]
    fn artboard_mobile_has_correct_dimensions() {
        let ab = Artboard::mobile("M");
        assert!((ab.width - 390.0).abs() < 1e-4);
        assert!((ab.height - 844.0).abs() < 1e-4);
    }

    #[test]
    fn canvas_component_serde_round_trip() {
        let c = CanvasComponent::new(
            ComponentKind::Button,
            "Test Button",
            Bounds::new(10.0, 20.0, 120.0, 44.0),
        );
        let json = serde_json::to_string(&c).expect("serialize");
        let restored: CanvasComponent = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(restored.name, c.name);
        assert_eq!(restored.bounds.x, c.bounds.x);
        assert_eq!(restored.bounds.width, c.bounds.width);
        assert!(matches!(restored.kind, ComponentKind::Button));
    }

    #[test]
    fn project_palette_default_has_all_fields() {
        let p = ProjectPalette::default();
        assert!(!p.primary.is_empty());
        assert!(!p.secondary.is_empty());
        assert!(!p.accent.is_empty());
        assert!(!p.success.is_empty());
        assert!(!p.error.is_empty());
    }

    #[test]
    fn component_id_display_is_uuid_string() {
        let id = ComponentId::new();
        let s = id.to_string();
        assert_eq!(s.len(), 36, "UUID string should be 36 chars");
        assert!(s.contains('-'), "UUID string should contain dashes");
    }

    #[test]
    fn gesture_phase_default_is_idle() {
        assert_eq!(GesturePhase::default(), GesturePhase::Idle);
    }
}
