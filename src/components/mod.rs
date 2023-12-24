pub mod sprite;
pub mod ui;

pub use general::*;

pub mod general {
    use bevy::{
        prelude::{
            Asset, AssetServer
        },
        asset::Handle,
        sprite::Anchor, math::{Vec2, Vec3, Quat}, transform::components::Transform, render::{color::Color, view::Visibility}, ui::{BackgroundColor, BorderColor}
    };
    use serde::{Deserialize, Serialize};

    use crate::prefab::IntoComponent;


    #[derive(Default, Debug, Clone, Deserialize, Serialize)]
    pub enum AnchorPrefab {
        #[default]
        Center,
        BottomLeft,
        BottomCenter,
        BottomRight,
        CenterLeft,
        CenterRight,
        TopLeft,
        TopCenter,
        TopRight,
        Custom(Vec2),
    } impl AnchorPrefab {
        pub fn into_anchor(self) -> Anchor {
            match self {
                AnchorPrefab::Center => Anchor::Center,
                AnchorPrefab::BottomLeft => Anchor::BottomLeft,
                AnchorPrefab::BottomCenter => Anchor::BottomCenter,
                AnchorPrefab::BottomRight => Anchor::BottomRight,
                AnchorPrefab::CenterLeft => Anchor::CenterLeft,
                AnchorPrefab::CenterRight => Anchor::CenterRight,
                AnchorPrefab::TopLeft => Anchor::TopLeft,
                AnchorPrefab::TopCenter => Anchor::TopCenter,
                AnchorPrefab::TopRight => Anchor::TopRight,
                AnchorPrefab::Custom(c) => Anchor::Custom(c),
            }
        }
    } impl IntoComponent for AnchorPrefab {
        type Component = Anchor;
        fn into_component(self) -> Self::Component {
            self.into_anchor()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum TransformPrefab {
        XYZ(f32, f32, f32),
        Quat(f32, f32, f32, f32),
        Scale(f32, f32, f32),
        Custom {
            #[serde(default)]
            translation: [f32; 3],
            #[serde(default)]
            scale: [f32; 3],
            #[serde(default)]
            rotation: [f32; 4]
        }
    } impl TransformPrefab {
        const IDENTITY: Self = Self::Custom {
            translation: [0., 0., 0.],
            scale: [1., 1., 1.],
            rotation: [0., 0., 0., 1.],
        };
    } impl Default for TransformPrefab {
        fn default() -> Self {
            Self::IDENTITY
        }
    } impl TransformPrefab {
        pub fn into_transform(self) -> Transform {
            match self {
                Self::XYZ(x, y, z) => {
                    Transform::from_xyz(x, y, z)
                }
                Self::Quat(x, y, z, w) => {
                    Transform::from_rotation(Quat::from_xyzw(x, y, z, w))
                }
                Self::Scale(x, y, z) => {
                    Transform::from_scale(Vec3::new(x, y, z))
                }
                Self::Custom { translation, scale, rotation } => {
                    Transform {
                        translation: Vec3::from_array(translation),
                        scale: Vec3::from_array(scale),
                        rotation: Quat::from_array(rotation)
                    }
                }
            }
        }
    } impl IntoComponent for TransformPrefab {
        type Component = Transform;
        fn into_component(self) -> Self::Component {
            self.into_transform()
        }
    }

    /// Prefab used for loading external assets
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum HandlePrefab<A: Asset> {
        File(String),

        #[serde(skip)]
        Loaded(Handle<A>),

    } impl <A: Asset>HandlePrefab<A> {
        pub fn load_self(&self, asset_server: &AssetServer) -> Option<Handle<A>> {
            match self {
                Self::File(s) => {
                    Some(asset_server.load(s.to_owned()))
                }
                Self::Loaded(_) => {
                    println!("[warn] Tried to load already loaded asset");
                    None
                }
            }
        }

        pub fn into_handle(self) -> Option<Handle<A>> {
            match self {
                HandlePrefab::File(_) => {
                    println!("[warn] Asset not yet loaded");
                    None
                }
                HandlePrefab::Loaded(h) => Some(h)
            }
        }
    }

    #[derive(Clone, Deserialize, Serialize)]
    pub struct BackgroundColorPrefab(pub Color);
    impl Default for BackgroundColorPrefab {
        fn default() -> Self {
            Self(Color::NONE)
        }
    } impl BackgroundColorPrefab {
        pub fn from_white() -> Self {
            Self(Color::WHITE)
        }
    } impl IntoComponent for BackgroundColorPrefab {
        type Component = BackgroundColor;
        fn into_component(self) -> Self::Component {
            BackgroundColor(self.0)
        }
    }

    #[derive(Clone, Deserialize, Serialize)]
    pub struct BorderColorPrefab(pub Color);
    impl Default for BorderColorPrefab {
        fn default() -> Self {
            Self(Color::NONE)
        }
    } impl BorderColorPrefab {
        pub fn from_white() -> Self {
            Self(Color::WHITE)
        }
    } impl IntoComponent for BorderColorPrefab {
        type Component = BorderColor;
        fn into_component(self) -> Self::Component {
            BorderColor(self.0)
        }
    }

    #[derive(Default, Clone, Debug, Deserialize, Serialize)]
    pub enum VisibilityPrefab {
        #[default]
        Inherited,
        Hidden,
        Visible,
    } impl VisibilityPrefab {
        pub fn into_visibility(self) -> Visibility {
            match self {
                Self::Inherited => Visibility::Inherited,
                Self::Hidden => Visibility::Hidden,
                Self::Visible => Visibility::Visible,
            }
        }
    } impl IntoComponent for VisibilityPrefab {
        type Component = Visibility;
        fn into_component(self) -> Self::Component {
            self.into_visibility()
        }
    }
}