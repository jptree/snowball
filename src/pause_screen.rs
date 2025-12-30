use bevy::{
    app::{App, Update}, color::{Color, palettes::css::CRIMSON}, ecs::{children, name::Name, system::ResMut}, state::{
        app::AppExtStates,
        state::{NextState, OnEnter, State, States},
        state_scoped::DespawnOnExit,
    }, text::{TextColor, TextFont}, ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, UiRect, percent, px}
};
#[cfg(not(target_arch = "wasm32"))]
use bevy::{
    ecs::system::{Commands, Res},
    input::{ButtonInput, keyboard::KeyCode},
    ui::{Node, widget::Text},
    utils::default,
};

use bevy::prelude::SpawnRelated;

use crate::GameState;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn toggle_pause_screen(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_game_state: Res<State<GameState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    use bevy::input::keyboard::KeyCode;

    if keyboard.just_pressed(KeyCode::Escape) {
        match current_game_state.get() {
            GameState::Game => game_state.set(GameState::Menu),
            GameState::Menu => {
                game_state.set(GameState::Game);
                menu_state.set(MenuState::Disabled);
            }
        }
    }
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    #[default]
    Disabled,
}

pub fn menu_plugin(app: &mut App) {
    app.init_state::<MenuState>()
        .add_systems(Update, toggle_pause_screen)
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_systems(OnEnter(MenuState::Main), main_menu_setup);
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(MenuState::Main),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        Name::new("Root"),
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            Name::new("Idk"),
            children![(
                Text::new("Bevy Game Menu UI"),
                TextFont {
                    font_size: 67.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::all(px(50)),
                    ..default()
                },
            )]
        )],
    ));
}

