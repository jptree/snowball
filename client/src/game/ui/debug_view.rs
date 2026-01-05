use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::game::player::camera_controller::CameraController;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct DebugCameraText;

pub fn debug_view_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut span in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
            && let Some(value) = fps.smoothed()
        {
            // Update the value of the second section
            **span = format!("{value:.2}");
        }
    }
}

pub fn update_debug_camera_text(
    camera: Single<&CameraController>,
    mut query: Query<&mut TextSpan, With<DebugCameraText>>,
) {
    for mut span in &mut query {
        // Update the value of the second section
        let x = camera.rotation.x;
        let y = camera.rotation.y;

        **span = format!("x: {x:.2} y: {y:.2}");
    }
}

pub fn setup(mut commands: Commands) {
    let fps = commands
        .spawn(Text::new("FPS: "))
        .with_child((TextSpan::default(), FpsText)).id();

    let camera= commands
        .spawn(Text::new("Camera: "))
        .with_child((TextSpan::default(), DebugCameraText)).id();

    let mut root = commands.spawn((
        Name::new("Root"),
        Node {
            flex_direction: FlexDirection::Column,
            ..default()
        }
    ));

    root.add_child(fps);
    root.add_child(camera);
}

// fn toggle_debug(
//     keyboard: Res<ButtonInput<KeyCode>>,
// ) {
//     if keyboard.just_pressed(KeyCode::Space) {
//         wireframe_config.global = !wireframe_config.global;
//     }
// }
