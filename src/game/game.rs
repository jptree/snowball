use bevy::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};

use crate::game::ui::ui;

use super::{level::level, player::player};
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            level::LevelPlugin,
            player::PlayerPlugin,
            ui::UiPlugin
        ));
    }
}