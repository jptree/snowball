use bevy::{ecs::component::Component, state::state::States};

pub mod pause_screen;
pub mod view_model;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Game,
    Menu,
}

#[derive(Component)]
pub struct Shape;