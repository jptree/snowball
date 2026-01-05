use std::f32::consts::{FRAC_2_PI, PI};

use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    Ccd, Collider, ColliderMassProperties, GravityScale, RigidBody, Velocity,
};
use snowball::uv_debug_texture;

use crate::{
    game::player::{
        camera_controller::CameraController,
        player::{Player, TracerSpawnSpot},
    },
};

pub fn throw_on_click(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    spawn_spot: Single<&GlobalTransform, With<TracerSpawnSpot>>,
    camera: Single<(&CameraController)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let inner = spawn_spot.into_inner();
    let camera = camera.into_inner();

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let camera_x = f32::sin(camera.rotation.x - PI);
    let camera_z = f32::cos(camera.rotation.x - PI);
    let camera_y = f32::sin(camera.rotation.y);

    let speed = 10.0;

    let throw_vector = Vec3::new(camera_x, camera_y, camera_z) * speed;

    if mouse_input.just_pressed(MouseButton::Left) {
        info!("Throwing!");
        commands
            .spawn((
                RigidBody::Dynamic,
                Mesh3d(meshes.add(Sphere::default())),
                MeshMaterial3d(debug_material.clone()),
                Collider::cuboid(1., 1., 1.),
            ))
            .insert(Transform::from_translation(inner.translation()))
            .insert(Velocity {
                linvel: throw_vector,
                angvel: Vec3::new(0.0, 0.0, 0.0),
            })
            .insert(GravityScale(1.0))
            .insert(ColliderMassProperties::Density(2.0))
            .insert(Ccd::enabled());
    }
}
