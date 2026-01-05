use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game::player::{camera_controller::CameraController, input::PlayerInput};

use super::player::Player;

use std::f32::consts::PI;

pub fn update_movement_input(keys: Res<ButtonInput<KeyCode>>, mut input: ResMut<PlayerInput>) {
    input.movement = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        input.movement.x += 1.;
    }
    if keys.pressed(KeyCode::KeyA) {
        input.movement.y -= 1.;
    }
    if keys.pressed(KeyCode::KeyS) {
        input.movement.x -= 1.;
    }
    if keys.pressed(KeyCode::KeyD) {
        input.movement.y += 1.;
    }

    if keys.just_pressed(KeyCode::Space) {
        input.movement.z = 1.;
    }
}

pub fn update_movement(
    time: Res<Time<Fixed>>,
    input: Res<PlayerInput>,
    mut player_query: Query<(
        &mut Player,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
    )>,
    camera_query: Single<&CameraController>,
) {
    let camera = camera_query.into_inner();

    for (mut player, mut controller, controller_output) in player_query.iter_mut() {
        if let Some(output) = controller_output {
            if output.grounded {
                player.velocity = Vec3::ZERO;

                // Can only jump on ground
                player.velocity.y += input.movement.z * 10.0;
            }
        }

        let camera_x = camera.rotation.x - PI;

        let forward = Vec2::new(f32::sin(camera_x), f32::cos(camera_x));

        let right = Vec2::new(-forward.y, forward.x);

        if let Some(movement_direction) =
            (forward * input.movement.x + right * input.movement.y).try_normalize()
        {
            player.velocity.x = movement_direction.x * player.speed;
            player.velocity.z = movement_direction.y * player.speed;
        }

        // Gravity
        player.velocity.y -= player.gravity * time.timestep().as_secs_f32();


        //delta
        controller.translation = Some(player.velocity * time.timestep().as_secs_f32());
    }
}
