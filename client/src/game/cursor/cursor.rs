use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .add_systems(Update, update_cursor_locking)
            .add_systems(Startup, init_cursor_properties);
    }
}

#[derive(Resource, Default)]
pub struct Cursor {
    locked: bool,
}

impl Cursor {
    pub fn invert_lock(&mut self, 
        window: &mut Mut<'_, Window>,
        cursor: &mut Mut<'_, CursorOptions>,
    ) {
        self.locked = !self.locked;
        cursor.visible = !self.locked;

        if self.locked {
            let window_width = window.width();
            let window_height = window.height();
            cursor.grab_mode = CursorGrabMode::Locked;
            window.set_cursor_position(Some(Vec2::new(window_width / 2., window_height / 2.)));
        } else {
            cursor.grab_mode = CursorGrabMode::None;
        }
    }
}

fn init_cursor_properties(
    window_query: Single<&mut Window, With<PrimaryWindow>>,
    mut cursor: ResMut<Cursor>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    let mut window = window_query.into_inner();
    cursor.invert_lock(&mut window, &mut cursor_options);
}

fn update_cursor_locking(
    keys: Res<ButtonInput<KeyCode>>,
    window_query: Single<&mut Window, With<PrimaryWindow>>,
    mut cursor: ResMut<Cursor>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    let mut window = window_query.into_inner();
    if keys.just_pressed(KeyCode::Escape) {
        cursor.invert_lock(&mut window, &mut cursor_options);
    }
}