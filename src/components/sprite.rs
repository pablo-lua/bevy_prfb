use bevy::{
    prelude::{
        AssetServer, Res
    },
    ecs::system::{RunSystemOnce, In},
    render::{texture::Image, color::Color}, sprite::{SpriteBundle, Sprite}, math::{Vec2, Rect}
};
use serde::{Deserialize, Serialize};

use crate::{PrefabData, IntoComponent, ColorPrefab};

use super::general::{AnchorPrefab, HandlePrefab, TransformPrefab, VisibilityPrefab};

// ARRUMAR PARA NÃO MAIS RUNAR ONE-SHOT SYSTEM
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SpriteBundlePrefab {
    #[serde(default)]
    sprite: SpritePrefab,
    texture: HandlePrefab<Image>,
    #[serde(default)]
    transform: TransformPrefab,
    #[serde(default)]
    visibility: VisibilityPrefab
} impl SpriteBundlePrefab {
    pub fn into_sprite_bundle(self) -> Option<SpriteBundle> {
        if let Some(handle) = self.texture.into_handle() {
            Some(SpriteBundle {
                sprite: self.sprite.into_sprite(),
                transform: self.transform.into_transform(),
                texture: handle,
                visibility: self.visibility.into_visibility(),
                ..Default::default()
            })
        } else {
            None
        }
    }
} impl PrefabData for SpriteBundlePrefab {
    fn insert_into_entity(self, entidade: &mut bevy::prelude::EntityWorldMut) {
        let spritebundle = self.into_sprite_bundle();
        if let Some(s) = spritebundle {
            entidade.insert(s);
        }
    }
    fn load_sub_assets(&mut self, _world: &mut bevy::prelude::World) -> bool {
        let handle = _world.run_system_once_with(self.texture.clone(), |In(texture): In<HandlePrefab<Image>>, asset_server: Res<AssetServer>| {
            texture.load_self(&asset_server)
        });
        if let Some(h) = handle {
            self.texture = HandlePrefab::Loaded(h);
            false
        } else {

            true
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SpritePrefab {
    /// The sprite's color tint
    pub color: ColorPrefab,
    /// Flip the sprite along the `X` axis
    pub flip_x: bool,
    /// Flip the sprite along the `Y` axis
    pub flip_y: bool,
    /// An optional custom size for the sprite that will be used when rendering, instead of the size
    /// of the sprite's image
    pub custom_size: Option<Vec2>,
    /// An optional rectangle representing the region of the sprite's image to render, instead of
    /// rendering the full image. This is an easy one-off alternative to using a texture atlas.
    pub rect: Option<Rect>,
    /// [`Anchor`] point of the sprite in the world
    pub anchor: AnchorPrefab,
} impl SpritePrefab {
    pub fn into_sprite(self) -> Sprite {
        Sprite {
            color: self.color.into_color(),
            flip_x: self.flip_x,
            flip_y: self.flip_y,
            custom_size: self.custom_size,
            rect: self.rect,
            anchor: self.anchor.into_anchor(),
        }
    }
} impl IntoComponent for SpritePrefab {
    type Component = Sprite;
    fn into_component(self) -> Self::Component {
        self.into_sprite()
    }
}