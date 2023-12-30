pub mod a11y;
pub mod sprite;
pub mod ui;

pub use general::*;

pub mod general {
    use bevy::{
        asset::Handle,
        math::{Quat, Vec2, Vec3},
        prelude::{Asset, AssetServer},
        render::{color::Color, view::Visibility},
        sprite::Anchor,
        transform::components::Transform,
        ui::{BackgroundColor, BorderColor},
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
    }
    impl AnchorPrefab {
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
    }
    impl IntoComponent for AnchorPrefab {
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
            rotation: [f32; 4],
        },
    }
    impl TransformPrefab {
        const IDENTITY: Self = Self::Custom {
            translation: [0., 0., 0.],
            scale: [1., 1., 1.],
            rotation: [0., 0., 0., 1.],
        };
    }
    impl Default for TransformPrefab {
        fn default() -> Self {
            Self::IDENTITY
        }
    }
    impl TransformPrefab {
        pub fn into_transform(self) -> Transform {
            match self {
                Self::XYZ(x, y, z) => Transform::from_xyz(x, y, z),
                Self::Quat(x, y, z, w) => Transform::from_rotation(Quat::from_xyzw(x, y, z, w)),
                Self::Scale(x, y, z) => Transform::from_scale(Vec3::new(x, y, z)),
                Self::Custom {
                    translation,
                    scale,
                    rotation,
                } => Transform {
                    translation: Vec3::from_array(translation),
                    scale: Vec3::from_array(scale),
                    rotation: Quat::from_array(rotation),
                },
            }
        }
    }
    impl IntoComponent for TransformPrefab {
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

        // When no file is expected, this will insert an Handle<A>::default in the entity
        None

    } impl <A: Asset>HandlePrefab<A> {
        pub fn load_self(&self, asset_server: &AssetServer) -> Option<Handle<A>> {
            match self {
                Self::File(s) => Some(asset_server.load(s.to_owned())),
                Self::Loaded(_) => {
                    println!("[warn] Tried to load already loaded asset");
                    None
                }
                Self::None => {
                    Some(Default::default())
                }
            }
        }

        pub fn into_handle(self) -> Option<Handle<A>> {
            match self {
                HandlePrefab::File(_) => {
                    println!("[warn] Asset not yet loaded");
                    None
                }
                HandlePrefab::Loaded(h) => Some(h),
                HandlePrefab::None => Some(Default::default())
            }
        }
    }

    #[derive(Clone, Deserialize, Serialize)]
    pub struct BackgroundColorPrefab(pub ColorPrefab);
    impl Default for BackgroundColorPrefab {
        fn default() -> Self {
            Self(ColorPrefab::Rgba(0., 0., 0., 0.))
        }
    }
    impl BackgroundColorPrefab {
        pub fn from_white() -> Self {
            Self(ColorPrefab::Rgba(1., 1., 1., 1.))
        }
    }
    impl IntoComponent for BackgroundColorPrefab {
        type Component = BackgroundColor;
        fn into_component(self) -> Self::Component {
            BackgroundColor(self.0.into_color())
        }
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub enum ColorPrefab {
        Rgba(f32, f32, f32, f32),
        RgbaLinear(f32, f32, f32, f32),
        Hsla(f32, f32, f32, f32),
        Lcha(f32, f32, f32, f32)
    } impl ColorPrefab {
        pub fn into_color(self) -> Color {
            match self {
                ColorPrefab::Hsla(h, s, l, a) => Color::hsla(h, s, l, a),
                ColorPrefab::Lcha(l, c, h, a) => Color::lcha(l, c, h, a),
                ColorPrefab::Rgba(r, g, b, a) => Color::rgba(r, g, b, a),
                ColorPrefab::RgbaLinear(r, g, b, a) => Color::rgba_linear(r, g, b, a)
            }
        }
    } impl Default for ColorPrefab {
        fn default() -> Self {
            Self::Rgba(1., 1., 1., 1.)
        }
    }

    #[derive(Clone, Deserialize, Serialize)]
    pub struct BorderColorPrefab(pub ColorPrefab);
    impl Default for BorderColorPrefab {
        fn default() -> Self {
            Self(ColorPrefab::Rgba(0., 0., 0., 0.))
        }
    }
    impl BorderColorPrefab {
        pub fn from_white() -> Self {
            Self(ColorPrefab::Rgba(1., 1., 1., 1.))
        }
    }
    impl IntoComponent for BorderColorPrefab {
        type Component = BorderColor;
        fn into_component(self) -> Self::Component {
            BorderColor(self.0.into_color())
        }
    }

    #[derive(Default, Clone, Debug, Deserialize, Serialize)]
    pub enum VisibilityPrefab {
        #[default]
        Inherited,
        Hidden,
        Visible,
    }
    impl VisibilityPrefab {
        pub fn into_visibility(self) -> Visibility {
            match self {
                Self::Inherited => Visibility::Inherited,
                Self::Hidden => Visibility::Hidden,
                Self::Visible => Visibility::Visible,
            }
        }
    }
    impl IntoComponent for VisibilityPrefab {
        type Component = Visibility;
        fn into_component(self) -> Self::Component {
            self.into_visibility()
        }
    }
}
