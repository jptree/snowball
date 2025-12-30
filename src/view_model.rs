use bevy::{
    app::{App, Startup, Update},
    asset::Assets,
    color::palettes,
    ecs::system::{Commands, ResMut},
    input::mouse::AccumulatedMouseMotion,
    math::{bounding::{Aabb2d, BoundingCircle, BoundingSphere, BoundingVolume, IntersectsVolume}, primitives::Cuboid},
    mesh::Mesh,
    pbr::StandardMaterial,
};

use bevy::{camera::visibility::RenderLayers, light::NotShadowCaster, prelude::*};

use std::f32::consts::FRAC_PI_2;

use crate::Shape;

#[derive(Debug, Component)]
struct Player;

#[derive(Debug, Component, Deref, DerefMut)]
struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(
            // These factors are just arbitrary mouse sensitivity values.
            // It's often nicer to have a faster horizontal sensitivity than vertical.
            // We use a component for them so that we can make them user-configurable at runtime
            // for accessibility reasons.
            // It also allows you to inspect them in an editor if you `Reflect` the component.
            Vec2::new(0.003, 0.002),
        )
    }
}

#[derive(Debug, Component)]
struct WorldModelCamera;

/// Used implicitly by all entities without a `RenderLayers` component.
/// Our world model camera and all objects other than the player are on this layer.
/// The light source belongs to both layers.
const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
const VIEW_MODEL_RENDER_LAYER: usize = 1;

fn check_for_collisions(
    collider_query: Query<(Entity, &Transform, Option<&Shape>), Without<Player>>,
    location: &Vec3
) -> bool {

    for (collider_entity, collider_transform, maybe_brick) in &collider_query {
        
        let collision = BoundingSphere::new(*location, 0.01).intersects(
            &BoundingSphere::new(collider_transform.translation, 0.1)
        );

        if collision {
            return true;
        }
    }

    false
}

fn move_player(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    collider_query: Query<(Entity, &Transform, Option<&Shape>), Without<Player>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();

    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        // Note that we are not multiplying by delta_time here.
        // The reason is that for mouse movement, we already get the full movement that happened since the last frame.
        // This means that if we multiply by delta_time, we will get a smaller rotation than intended by the user.
        // This situation is reversed when reading e.g. analog input from a gamepad however, where the same rules
        // as for keyboard input apply. Such an input should be multiplied by delta_time to get the intended rotation
        // independent of the framerate.
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

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

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }

    // let target = Vec3::ZERO;
    // let mut delta_forward = 0.0;
    let speed = 0.5;
    let mut movement = Vec3::new(0.0, 0.0, 0.0);

    let mut delta_forward = 0.0;
    let mut delta_left = 0.0;
    if input.pressed(KeyCode::KeyW) {
        // movement += speed * transform.forward();
        // transform.translation += m;
        delta_forward += 1.0;
    }

    if input.pressed(KeyCode::KeyS) {
        // let m = speed * transform.back();
        // transform.translation += m;
        delta_forward -= 1.0;
    }

    if input.pressed(KeyCode::KeyA) {
        // let m = speed * transform.left();
        // transform.translation += m;
        delta_left += 1.0;
    }

    if input.pressed(KeyCode::KeyD) {
        // let m = speed * transform.right();
        // transform.translation += m;
        delta_left -= 1.0;
    }

    let speed = 5.0;
    // delta_forward *= speed * time.delta_secs();

    let fwd = speed * transform.forward();
    let left = speed * transform.left();
    let new_translation = transform.translation + fwd * delta_forward * time.delta_secs() + left * delta_left * time.delta_secs();
    // if !check_for_collisions(collider_query, &new_translation) {
        transform.translation = new_translation;
    // }

}

fn change_fov(
    input: Res<ButtonInput<KeyCode>>,
    mut world_model_projection: Single<&mut Projection, With<WorldModelCamera>>,
) {
    let Projection::Perspective(perspective) = world_model_projection.as_mut() else {
        unreachable!(
            "The `Projection` component was explicitly built with `Projection::Perspective`"
        );
    };

    if input.pressed(KeyCode::ArrowUp) {
        perspective.fov -= 1.0_f32.to_radians();
        perspective.fov = perspective.fov.max(20.0_f32.to_radians());
    }
    if input.pressed(KeyCode::ArrowDown) {
        perspective.fov += 1.0_f32.to_radians();
        perspective.fov = perspective.fov.min(160.0_f32.to_radians());
    }
}

pub fn view_model_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_view_model)
        .add_systems(Update, (move_player, change_fov));
}

fn spawn_view_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(palettes::css::ALICE_BLUE));

    commands.spawn((
        Player,
        CameraSensitivity::default(),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Visibility::default(),
        children![
            (
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }),
            ),
            // Spawn view model camera.
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
            // Spawn the player's right arm.
            (
                Mesh3d(arm),
                MeshMaterial3d(arm_material),
                Transform::from_xyz(0.2, -0.1, -0.25),
                // Ensure the arm is only rendered by the view model camera.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                // The arm is free-floating, so shadows would look weird.
                NotShadowCaster,
            ),
        ],
    ));
}
