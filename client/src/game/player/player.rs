use bevy::{camera::visibility::RenderLayers, color::palettes, light::NotShadowCaster, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::game::{game::VIEW_MODEL_RENDER_LAYER, player::{input::PlayerInput, player_throw::throw_on_click}};

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
                    throw_on_click
                ),
            )
            // physics timestep
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

#[derive(Component)]
pub struct TracerSpawnSpot;

fn init_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
                sensitivity: Vec2::new(0.003, 0.002),
                rotation: Vec2::ZERO,
                rotation_lock: 88.0,
            },
        ))
        .id();

    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(palettes::tailwind::AMBER_950));

    let player_entity = commands
        .spawn((
            Player {
                velocity: Vec3::ZERO,
                gravity: 9.8,
                speed: 20.0,
            },
            Transform::from_translation(Vec3::new(10., 10., 10.)),
            Collider::capsule(Vec3::new(1., 1., 1.), Vec3::new(1., 1., 1.), 1.),
            LockedAxes::ROTATION_LOCKED,
            RigidBody::Dynamic,
            KinematicCharacterController {
                up: Vec3::Y,
                offset: CharacterLength::Absolute(0.01),
                ..default()
            },
            children![
                (
                    Mesh3d(arm),
                    MeshMaterial3d(arm_material),
                    Transform::from_xyz(0.2, -0.1, -0.25),
                    // Ensure the arm is only rendered by the view model camera.
                    RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                    // The arm is free-floating, so shadows would look weird.
                    NotShadowCaster,
                ),
                // View model camera
                (
                    Camera3d::default(),
                    Camera {
                        // Bump the order to render on top of the world model.
                        order: 1,
                        ..default()
                    },
                    Projection::from(PerspectiveProjection {
                        fov: 70.0_f32.to_radians(),
                        ..default()
                    }),
                    // Only render objects belonging to the view model.
                    RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                ),
                (
                    Transform::from_translation(Vec3::new(2., 2., 2.)),
                    TracerSpawnSpot
                )
            ],
        ))
        .id();

    // // commands.entity(camera_entity).push_children(&[tracer_spawn_entity,gun_entity]);
    commands.entity(player_entity).add_child(camera_entity);
}
