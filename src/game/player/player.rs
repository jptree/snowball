use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game::player::input::PlayerInput;

use super::{camera_controller, player_movement::*};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInput>()
            .add_systems(
                Update,
                (
                    update_movement_input,
                    camera_controller::update_camera_controller,
                ),
            )
            //physics timestep
            .add_systems(FixedUpdate, update_movement)
            .add_systems(Startup, init_player);
    }
}

#[derive(Component)]
pub struct Player {
    pub velocity: Vec3,
    pub gravity: f32,
    pub speed: f32,
}

fn init_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fov = 103.0_f32.to_radians();
    let camera_entity = commands
        .spawn((
            Camera3d::default(),
            Transform::IDENTITY,
            Projection::Perspective(PerspectiveProjection {
                fov: fov,
                ..default()
            }),
            camera_controller::CameraController {
                sensitivity: Vec2::new(0.03, 0.02),
                rotation: Vec2::ZERO,
                rotation_lock: 88.0,
            },
        ))
        .id();

    let player_entity = commands
        .spawn((
            Player {
                velocity: Vec3::ZERO,
                gravity: 9.8,
                speed: 20.0,
            },
            Transform::from_translation(Vec3::new(10., 10., 10.)),
            Collider::cuboid(1., 1., 1.),
            RigidBody::Dynamic,
            KinematicCharacterController {
                up: Vec3::Y,
                offset: CharacterLength::Absolute(0.01),
                ..default()
            },
            ColliderMassProperties::Density(2.0)
        ))
        .id();

    // // commands.entity(camera_entity).push_children(&[tracer_spawn_entity,gun_entity]);
    commands.entity(player_entity).add_child(camera_entity);
}
