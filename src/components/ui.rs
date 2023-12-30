use bevy::{
    asset::AssetServer,
    log::warn,
    render::{color::Color, texture::Image},
    text::{BreakLineOn, Font, Text, TextAlignment, TextSection, TextStyle},
    ui::{
        node_bundles::{ButtonBundle, ImageBundle, NodeBundle, TextBundle},
        widget::Label,
        AlignContent, AlignItems, AlignSelf, Direction, Display, FlexDirection, FlexWrap,
        FocusPolicy, GridAutoFlow, GridPlacement, GridTrack, JustifyContent, JustifyItems,
        JustifySelf, Overflow, PositionType, RepeatedGridTrack, Style, UiImage, UiRect, Val,
        ZIndex,
    },
};
use bevy_prfb_macro::PrefabData;
use serde::{Deserialize, Serialize};

use crate::{prefab::{IntoComponent, PrefabData}, ColorPrefab};

use super::general::{HandlePrefab, BackgroundColorPrefab, BorderColorPrefab, TransformPrefab, VisibilityPrefab};

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct NodeBundlePrefab {
    pub style: StylePrefab,
    pub background_color: BackgroundColorPrefab,
    pub border_color: BorderColorPrefab,
    pub focus_policy: FocusPolicy,
    pub visibility: VisibilityPrefab,
    pub z_index: ZIndexPrefab,
}
impl Default for NodeBundlePrefab {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            border_color: Default::default(),
            style: Default::default(),
            focus_policy: Default::default(),
            visibility: Default::default(),
            z_index: Default::default(),
        }
    }
}
impl PrefabData for NodeBundlePrefab {
    fn insert_into_entity(self, entidade: &mut bevy::prelude::EntityWorldMut) {
        let node = NodeBundle {
            style: self.style.into_component(),
            background_color: self.background_color.into_component(),
            border_color: self.border_color.into_component(),
            focus_policy: self.focus_policy,
            visibility: self.visibility.into_component(),
            z_index: self.z_index.into_component(),
            ..Default::default()
        };
        entidade.insert(node);
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StylePrefab {
    pub display: Display,
    pub position_type: PositionType,
    pub overflow: Overflow,
    pub direction: Direction,
    pub left: Val,
    pub right: Val,
    pub top: Val,
    pub bottom: Val,
    pub width: Val,
    pub height: Val,
    pub min_width: Val,
    pub min_height: Val,
    pub max_width: Val,
    pub max_height: Val,
    pub aspect_ratio: Option<f32>,
    pub align_items: AlignItems,
    pub justify_items: JustifyItems,
    pub align_self: AlignSelf,
    pub justify_self: JustifySelf,
    pub align_content: AlignContent,
    pub justify_content: JustifyContent,
    pub margin: UiRect,
    pub padding: UiRect,
    pub border: UiRect,
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Val,
    pub row_gap: Val,
    pub column_gap: Val,
    pub grid_auto_flow: GridAutoFlow,
    pub grid_template_rows: Vec<RepeatedGridTrack>,
    pub grid_template_columns: Vec<RepeatedGridTrack>,
    pub grid_auto_rows: Vec<GridTrack>,
    pub grid_auto_columns: Vec<GridTrack>,
    pub grid_row: GridPlacement,
    pub grid_column: GridPlacement,
}
impl IntoComponent for StylePrefab {
    type Component = Style;
    fn into_component(self) -> Self::Component {
        Style {
            display: self.display,
            position_type: self.position_type,
            overflow: self.overflow,
            direction: self.direction,
            left: self.left,
            right: self.right,
            top: self.top,
            bottom: self.bottom,
            width: self.width,
            height: self.height,
            min_width: self.min_width,
            min_height: self.min_height,
            max_width: self.max_width,
            max_height: self.max_height,
            aspect_ratio: self.aspect_ratio,
            align_items: self.align_items,
            justify_items: self.justify_items,
            align_self: self.align_self,
            justify_self: self.justify_self,
            align_content: self.align_content,
            justify_content: self.justify_content,
            margin: self.margin,
            padding: self.padding,
            border: self.border,
            flex_direction: self.flex_direction,
            flex_wrap: self.flex_wrap,
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            flex_basis: self.flex_basis,
            row_gap: self.row_gap,
            column_gap: self.column_gap,
            grid_auto_flow: self.grid_auto_flow,
            grid_template_rows: self.grid_template_rows,
            grid_template_columns: self.grid_template_columns,
            grid_auto_rows: self.grid_auto_rows,
            grid_auto_columns: self.grid_auto_columns,
            grid_row: self.grid_row,
            grid_column: self.grid_column,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub enum ZIndexPrefab {
    Local(i32),
    Global(i32),
}
impl IntoComponent for ZIndexPrefab {
    type Component = ZIndex;
    fn into_component(self) -> Self::Component {
        match self {
            Self::Local(l) => ZIndex::Local(l),
            Self::Global(g) => ZIndex::Global(g),
        }
    }
}
impl Default for ZIndexPrefab {
    fn default() -> Self {
        Self::Local(0)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TextBundlePrefab {
    #[serde(default)]
    pub style: StylePrefab,

    pub text: TextPrefab,

    #[serde(default)]
    pub focus_policy: FocusPolicy,

    #[serde(default)]
    pub visibility: VisibilityPrefab,

    #[serde(default)]
    pub z_index: ZIndexPrefab,

    #[serde(default)]
    pub background_color: BackgroundColorPrefab,
}
impl Default for TextBundlePrefab {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            style: Default::default(),
            text: Default::default(),
            focus_policy: Default::default(),
            visibility: Default::default(),
            z_index: Default::default(),
        }
    }
}
impl PrefabData for TextBundlePrefab {
    fn insert_into_entity(self, entidade: &mut bevy::prelude::EntityWorldMut) {
        let text_bundle = TextBundle {
            background_color: self.background_color.into_component(),
            style: self.style.into_component(),
            text: self.text.into_text(),
            focus_policy: self.focus_policy,
            visibility: self.visibility.into_component(),
            z_index: self.z_index.into_component(),
            ..Default::default()
        };
        entidade.insert(text_bundle);
    }
    fn load_sub_assets(&mut self, _world: &mut bevy::prelude::World) -> bool {
        let server: Option<&AssetServer> = _world.get_resource();
        let Some(asset_server) = server else {
            warn!(target: "structs::ui", "AssetServer doesn't exist");
            return true;
        };
        self.text.load(asset_server)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TextPrefab {
    pub sections: Vec<TextSectionPrefab>,
    #[serde(default)]
    pub alignment: TextAlignment,
    #[serde(default = "def_break")]
    pub linebreak_behavior: BreakLineOn,
    pub default_font: Option<HandlePrefab<Font>>,
}
impl TextPrefab {
    pub fn into_text(mut self) -> Text {
        let sections: Vec<TextSection> = self
            .sections
            .drain(0..self.sections.len())
            .filter_map(|section: TextSectionPrefab| {
                section.into_section(self.default_font.clone())
            })
            .collect();
        Text {
            sections,
            alignment: self.alignment,
            linebreak_behavior: self.linebreak_behavior
        }
    }

    pub fn load(&mut self, asset_server: &AssetServer) -> bool {
        let mut loaded = false;
        if let Some(def_font) = self.default_font.as_mut() {
            let def_handle = def_font.load_self(&asset_server);
            if let Some(dh) = def_handle {
                *def_font = HandlePrefab::Loaded(dh);
            } else {
                loaded = true;
            }
        }
        for section in self.sections.iter_mut() {
            loaded |= section.style.load_self(asset_server);
        }
        loaded
    }
}
impl Default for TextPrefab {
    fn default() -> Self {
        Self {
            sections: Default::default(),
            alignment: TextAlignment::Left,
            linebreak_behavior: BreakLineOn::WordBoundary,
            default_font: None,
        }
    }
}

fn def_break() -> BreakLineOn {
    BreakLineOn::WordBoundary
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TextSectionPrefab {
    pub text: String,
    pub style: TextStylePrefab,
}
impl TextSectionPrefab {
    pub fn into_section(self, def_handle: Option<HandlePrefab<Font>>) -> Option<TextSection> {
        let style = self.style.into_text_style(def_handle);
        let Some(s) = style else {
            println!("[warn] No font given");
            return None;
        };
        Some(TextSection {
            value: self.text,
            style: s,
        })
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TextStylePrefab {
    /// A font must be defined at least in one place.
    font: Option<HandlePrefab<Font>>,

    #[serde(default = "default_font_size")]
    font_size: f32,
    #[serde(default)]
    color: ColorPrefab
} impl TextStylePrefab {
    pub fn into_text_style(self, def_handle: Option<HandlePrefab<Font>>) -> Option<TextStyle> {
        let handle = if let Some(h) = self.font {
            h.into_handle()
        } else if let Some(h) = def_handle {
            h.into_handle()
        } else {
            return None;
        };
        let Some(font) = handle else {
            return None;
        };
        Some(
            TextStyle {
                font,
                font_size: self.font_size,
                color: self.color.into_color()
            }
        )
    }
    pub fn load_self(&mut self, asset_server: &AssetServer) -> bool {
        if let Some(h) = self.font.as_ref() {
            if let Some(handle) = h.load_self(asset_server) {
                self.font = Some(HandlePrefab::Loaded(handle));
                false
            } else {
                true
            }
        } else {
            false
        }
    }
}

fn default_font_size() -> f32 {
    12.
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ImageBundlePrefab {
    #[serde(default)]
    pub style: StylePrefab,
    #[serde(default = "BackgroundColorPrefab::from_white")]
    pub background_color: BackgroundColorPrefab,
    pub image: UiImagePrefab,
    #[serde(default)]
    pub focus_policy: FocusPolicy,
    #[serde(default)]
    pub transform: TransformPrefab,
    #[serde(default)]
    pub visibility: VisibilityPrefab,
    #[serde(default)]
    pub z_index: ZIndexPrefab,
}
impl PrefabData for ImageBundlePrefab {
    fn insert_into_entity(self, entidade: &mut bevy::prelude::EntityWorldMut) {
        let Some(image) = self.image.into_image() else {
            warn!(target: "structs::ui", "UiImage with no image found");
            return;
        };
        let image_bundle = ImageBundle {
            image,
            style: self.style.into_component(),
            background_color: self.background_color.into_component(),
            focus_policy: self.focus_policy,
            transform: self.transform.into_component(),
            visibility: self.visibility.into_component(),
            z_index: self.z_index.into_component(),
            ..Default::default()
        };
        entidade.insert(image_bundle);
    }

    fn load_sub_assets(&mut self, _world: &mut bevy::prelude::World) -> bool {
        let asset_server: Option<&AssetServer> = _world.get_resource();
        let Some(asset_server) = asset_server else {
            warn!(target: "structs::ui", "AssetServer doesn't exist");
            return true;
        };
        self.image.load(asset_server)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UiImagePrefab {
    texture: HandlePrefab<Image>,

    #[serde(default)]
    flip_x: bool,

    #[serde(default)]
    flip_y: bool,
}
impl UiImagePrefab {
    pub fn into_image(self) -> Option<UiImage> {
        if let Some(ui_image) = self.texture.into_handle() {
            Some(UiImage {
                texture: ui_image,
                flip_x: self.flip_x,
                flip_y: self.flip_y,
            })
        } else {
            None
        }
    }

    pub fn load(&mut self, asset_server: &AssetServer) -> bool {
        if let Some(handle) = self.texture.load_self(asset_server) {
            self.texture = HandlePrefab::Loaded(handle);
            false
        } else {
            true
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ButtonBundlePrefab {
    #[serde(default)]
    pub style: StylePrefab,
    #[serde(default)]
    pub focus_policy: FocusPolicy,
    #[serde(default = "BackgroundColorPrefab::from_white")]
    pub background_color: BackgroundColorPrefab,
    #[serde(default)]
    pub border_color: BorderColorPrefab,
    #[serde(default)]
    pub image: Option<UiImagePrefab>,
    #[serde(default)]
    pub transform: TransformPrefab,
    #[serde(default)]
    pub visibility: VisibilityPrefab,
    #[serde(default)]
    pub z_index: ZIndexPrefab,
}
impl PrefabData for ButtonBundlePrefab {
    fn insert_into_entity(self, entidade: &mut bevy::prelude::EntityWorldMut) {
        let ui_image = if let Some(image) = self.image {
            if let Some(ui_img) = image.into_image() {
                Some(ui_img)
            } else {
                warn!(target: "structs::ui", "Tried to load UiImage, but failed");
                None
            }
        } else {
            None
        };

        let ui_image = ui_image.unwrap_or_default();
        let button_bundle = ButtonBundle {
            style: self.style.into_component(),
            focus_policy: self.focus_policy,
            background_color: self.background_color.into_component(),
            border_color: self.border_color.into_component(),
            image: ui_image,
            transform: self.transform.into_component(),
            visibility: self.visibility.into_component(),
            z_index: self.z_index.into_component(),
            ..Default::default()
        };
        entidade.insert(button_bundle);
    }

    fn load_sub_assets(&mut self, _world: &mut bevy::prelude::World) -> bool {
        let Some(ui_img) = self.image.as_mut() else {
            //Nothing to load
            return false;
        };
        let asset_server: Option<&AssetServer> = _world.get_resource();
        let Some(asset_server) = asset_server else {
            warn!(target: "structs::ui", "AssetServer doesn't exist");
            return true;
        };
        ui_img.load(asset_server)
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename = "Label")]
pub struct LabelPrefab;
impl IntoComponent for LabelPrefab {
    type Component = Label;
    fn into_component(self) -> Self::Component {
        Label
    }
}

impl IntoComponent for FocusPolicy {
    type Component = Self;
    fn into_component(self) -> Self::Component {
        self
    }
}

#[derive(PrefabData, Clone, Deserialize, Serialize)]
pub struct Common {
    style: StylePrefab,
    focus_policy: FocusPolicy,
    visibility: VisibilityPrefab,
    z_index: ZIndexPrefab,
}

// Add the needed data to render the text
// This will not add the flags the text need to render, only the text

impl PrefabData for TextPrefab {
    fn insert_into_entity(self, entity: &mut bevy::prelude::EntityWorldMut) {
        let text = self.into_text();
        entity.insert(text);
    }
    fn load_sub_assets(&mut self, _world: &mut bevy::prelude::World) -> bool {
        let Some(asset_server): Option<&AssetServer> = _world.get_resource() else {
            warn!(target: "ui", "AssetServer doesn't exist");
            return true;
        };
        self.load(asset_server)
    }
}

impl PrefabData for UiImagePrefab {
    fn insert_into_entity(self, entity: &mut bevy::prelude::EntityWorldMut) {
        if let Some(image) = self.into_image() {
            entity.insert(image);
        }
    }

    fn load_sub_assets(&mut self, _world: &mut bevy::prelude::World) -> bool {
        let Some(asset_server): Option<&AssetServer> = _world.get_resource() else {
            warn!(target: "ui", "AssetServer doesn't exist");
            return true;
        };
        self.load(asset_server)
    }
}

#[derive(Clone, Deserialize, Serialize, PrefabData)]
pub struct General {
    text_data: Option<TextPrefab>,
    image_data: Option<UiImagePrefab>,
}
