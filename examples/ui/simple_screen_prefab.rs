use bevy_prfb::{*, ui::{CreateUiExt, UiCreator}};
use bevy::prelude::*;
//use serde::{Deserialize, Serialize};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
pub struct MainMenu;

fn setup(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
    let prefab = cmd.load_ui(UiCreator::load_simple("assets/ui/menu.ron"));
    prefab.prepare_spawn(MainMenu);
}