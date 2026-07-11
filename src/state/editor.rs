use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Device preview mode ───────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DeviceMode {
    Mobile,
    Tablet,
    #[default]
    Desktop,
}

// ── Panel visibility state ────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PanelState {
    #[default]
    Expanded,
    Collapsed,
    Hidden,
}

// ── Active editor tool ────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ActiveTool {
    #[default]
    Select,
    Hand,
}

// ── Canvas transform (zoom + pan) ─────────────────────────────────────────────
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CanvasTransform {
    pub zoom: f64,
    pub offset_x: f64,
    pub offset_y: f64,
}

impl Default for CanvasTransform {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}

impl CanvasTransform {
    pub const ZOOM_MIN: f64 = 0.1;
    pub const ZOOM_MAX: f64 = 8.0;

    pub fn zoom_percent(&self) -> u32 {
        (self.zoom * 100.0).round() as u32
    }

    pub fn css_transform(&self) -> String {
        format!(
            "translate({}px, {}px) scale({})",
            self.offset_x, self.offset_y, self.zoom
        )
    }

    pub fn apply_zoom(&self, delta: f64, anchor_x: f64, anchor_y: f64) -> Self {
        let new_zoom = (self.zoom * (1.0 + delta * 0.1)).clamp(Self::ZOOM_MIN, Self::ZOOM_MAX);
        let scale_change = new_zoom / self.zoom;
        Self {
            zoom: new_zoom,
            offset_x: anchor_x - (anchor_x - self.offset_x) * scale_change,
            offset_y: anchor_y - (anchor_y - self.offset_y) * scale_change,
        }
    }
}

// ── Component kind — what can be placed on the canvas ─────────────────────────
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentKind {
    Button,
    TextBlock,
    Image,
    Container,
    Input,
    Divider,
    Spacer,
    NavigationBar,
    Card,
    List,
    Form,
    Modal,
}

impl ComponentKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Button => "Button",
            Self::TextBlock => "Text",
            Self::Image => "Image",
            Self::Container => "Container",
            Self::Input => "Input",
            Self::Divider => "Divider",
            Self::Spacer => "Spacer",
            Self::NavigationBar => "Nav Bar",
            Self::Card => "Card",
            Self::List => "List",
            Self::Form => "Form",
            Self::Modal => "Modal",
        }
    }

    pub fn category(&self) -> &'static str {
        match self {
            Self::Button | Self::Input => "Controls",
            Self::TextBlock => "Typography",
            Self::Image | Self::Card => "Media",
            Self::Container | Self::Divider | Self::Spacer => "Layout",
            Self::NavigationBar => "Navigation",
            Self::List | Self::Form | Self::Modal => "Forms",
        }
    }

    pub fn icon_path(&self) -> &'static str {
        match self {
            Self::Button => "M3 10h14v4H3z M5 10V6h10v4",
            Self::TextBlock => "M3 5h18M3 10h12M3 15h18M3 20h9",
            Self::Image => "M3 3h18v18H3z M3 15l5-5 4 4 3-3 5 5",
            Self::Container => "M3 3h18v18H3z",
            Self::Input => "M3 8h18v8H3z M7 12h2",
            Self::Divider => "M3 12h18",
            Self::Spacer => "M12 3v18M3 12h18",
            Self::NavigationBar => "M3 4h18v4H3z M6 6h.01M10 6h.01M14 6h.01",
            Self::Card => "M3 5h18v14H3z M3 9h18",
            Self::List => "M3 6h.01M7 6h10M3 12h.01M7 12h10M3 18h.01M7 18h10",
            Self::Form => "M3 3h18v18H3z M7 8h10M7 12h10M7 16h6",
            Self::Modal => "M5 3h14v18H5z M3 7h18M3 17h18",
        }
    }
}

// ── Canvas element ─────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasElement {
    pub id: Uuid,
    pub kind: ComponentKind,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub label: String,
    pub z_index: i32,
}

impl CanvasElement {
    pub fn new(kind: ComponentKind, x: f64, y: f64) -> Self {
        let (w, h) = match &kind {
            ComponentKind::Button => (120.0, 40.0),
            ComponentKind::TextBlock => (200.0, 40.0),
            ComponentKind::Image => (240.0, 160.0),
            ComponentKind::Container => (320.0, 240.0),
            ComponentKind::Input => (200.0, 40.0),
            ComponentKind::Divider => (240.0, 1.0),
            ComponentKind::Spacer => (120.0, 24.0),
            ComponentKind::NavigationBar => (375.0, 56.0),
            ComponentKind::Card => (280.0, 180.0),
            ComponentKind::List => (280.0, 160.0),
            ComponentKind::Form => (320.0, 280.0),
            ComponentKind::Modal => (360.0, 480.0),
        };
        let label = kind.label().to_string();
        Self {
            id: Uuid::new_v4(),
            kind,
            x,
            y,
            width: w,
            height: h,
            label,
            z_index: 0,
        }
    }
}

// ── Central editor state ──────────────────────────────────────────────────────
// All fields are RwSignal<T> which is Copy+Clone in Leptos 0.8, so EditorState
// can be Copy — this lets it be freely used across closures without .clone() noise.
#[derive(Clone, Copy)]
pub struct EditorState {
    pub canvas_transform: RwSignal<CanvasTransform>,
    pub panel_left: RwSignal<PanelState>,
    pub panel_right: RwSignal<PanelState>,
    pub panel_bottom: RwSignal<PanelState>,
    pub active_tool: RwSignal<ActiveTool>,
    pub device_mode: RwSignal<DeviceMode>,
    pub selected_ids: RwSignal<Vec<Uuid>>,
    pub elements: RwSignal<Vec<CanvasElement>>,
    pub history: RwSignal<Vec<Vec<CanvasElement>>>,
    pub history_index: RwSignal<usize>,
    pub is_dragging: RwSignal<bool>,
    pub dragging_kind: RwSignal<Option<ComponentKind>>,
    pub theme: RwSignal<&'static str>,
}

impl EditorState {
    pub fn new() -> Self {
        let elements: RwSignal<Vec<CanvasElement>> = RwSignal::new(vec![]);
        let snapshot = elements.get_untracked();
        Self {
            canvas_transform: RwSignal::new(CanvasTransform::default()),
            panel_left: RwSignal::new(PanelState::Expanded),
            panel_right: RwSignal::new(PanelState::Expanded),
            panel_bottom: RwSignal::new(PanelState::Collapsed),
            active_tool: RwSignal::new(ActiveTool::Select),
            device_mode: RwSignal::new(DeviceMode::Desktop),
            selected_ids: RwSignal::new(vec![]),
            elements,
            history: RwSignal::new(vec![snapshot]),
            history_index: RwSignal::new(0),
            is_dragging: RwSignal::new(false),
            dragging_kind: RwSignal::new(None),
            theme: RwSignal::new("dark"),
        }
    }

    pub fn zoom_percent(&self) -> Memo<u32> {
        let transform = self.canvas_transform;
        Memo::new(move |_| transform.get().zoom_percent())
    }

    pub fn push_to_history(&self) {
        let snapshot = self.elements.get_untracked();
        self.history.update(|h| {
            let idx = self.history_index.get_untracked();
            h.truncate(idx + 1);
            h.push(snapshot);
        });
        self.history_index
            .update(|i| *i = self.history.get_untracked().len().saturating_sub(1));
    }

    pub fn undo(&self) {
        let idx = self.history_index.get_untracked();
        if idx > 0 {
            let new_idx = idx - 1;
            self.history_index.set(new_idx);
            let snapshot = self.history.get_untracked()[new_idx].clone();
            self.elements.set(snapshot);
        }
    }

    pub fn redo(&self) {
        let idx = self.history_index.get_untracked();
        let len = self.history.get_untracked().len();
        if idx + 1 < len {
            let new_idx = idx + 1;
            self.history_index.set(new_idx);
            let snapshot = self.history.get_untracked()[new_idx].clone();
            self.elements.set(snapshot);
        }
    }

    pub fn can_undo(&self) -> Memo<bool> {
        let index = self.history_index;
        Memo::new(move |_| index.get() > 0)
    }

    pub fn can_redo(&self) -> Memo<bool> {
        let index = self.history_index;
        let history = self.history;
        Memo::new(move |_| index.get() + 1 < history.get().len())
    }

    pub fn drop_component(&self, kind: ComponentKind, canvas_x: f64, canvas_y: f64) {
        let element = CanvasElement::new(kind, canvas_x, canvas_y);
        self.elements.update(|els| {
            let z = els.len() as i32;
            let mut el = element;
            el.z_index = z;
            els.push(el);
        });
        self.push_to_history();
    }

    pub fn select(&self, id: Uuid, multi: bool) {
        self.selected_ids.update(|ids| {
            if multi {
                if ids.contains(&id) {
                    ids.retain(|i| *i != id);
                } else {
                    ids.push(id);
                }
            } else {
                *ids = vec![id];
            }
        });
    }

    pub fn clear_selection(&self) {
        self.selected_ids.set(vec![]);
    }

    pub fn delete_selected(&self) {
        let selected = self.selected_ids.get_untracked();
        self.elements
            .update(|els| els.retain(|e| !selected.contains(&e.id)));
        self.selected_ids.set(vec![]);
        self.push_to_history();
    }

    pub fn toggle_theme(&self) {
        self.theme.update(|t| {
            *t = if *t == "dark" { "light" } else { "dark" };
        });
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}
