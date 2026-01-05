use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::{prelude::*};

#[derive(Component)]
pub struct CameraController {
    pub rotation: Vec2,
    pub rotation_lock: f32,
    pub sensitivity: Vec2,
}

use std::f32::consts::FRAC_PI_2;

pub fn update_camera_controller(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &mut CameraController)>,
) {
    let (mut transform, mut camera_controller) = player.into_inner();

    let delta = accumulated_mouse_motion.delta;

    let camera_sensitivity_x = camera_controller.sensitivity.x;
    let camera_sensitivity_y= camera_controller.sensitivity.y;

    if delta != Vec2::ZERO {
        // Note that we are not multiplying by delta_time here.
        // The reason is that for mouse movement, we already get the full movement that happened since the last frame.
        // This means that if we multiply by delta_time, we will get a smaller rotation than intended by the user.
        // This situation is reversed when reading e.g. analog input from a gamepad however, where the same rules
        // as for keyboard input apply. Such an input should be multiplied by delta_time to get the intended rotation
        // independent of the framerate.
        let delta_yaw = -delta.x * camera_sensitivity_x;
        let delta_pitch = -delta.y * camera_sensitivity_y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        // If the pitch was ±¹⁄₂ π, the camera would look straight up or down.
        // When the user wants to move the camera back to the horizon, which way should the camera face?
        // The camera has no way of knowing what direction was "forward" before landing in that extreme position,
        // so the direction picked will for all intents and purposes be arbitrary.
        // Another issue is that for mathematical reasons, the yaw will effectively be flipped when the pitch is at the extremes.
        // To not run into these issues, we clamp the pitch to a safe range.
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        camera_controller.rotation.x = yaw;
        camera_controller.rotation.y = pitch;

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

    }
    
}