use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

use crate::game::ui::debug_view::{debug_view_system, setup, update_debug_camera_text};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, (debug_view_system, update_debug_camera_text));
    }
}
