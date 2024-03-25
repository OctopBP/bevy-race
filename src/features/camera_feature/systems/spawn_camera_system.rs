use bevy::prelude::*;
use crate::features::camera_feature::components::camera_component::Camera;

pub fn spawn_camera_system(
    mut commands: Commands,
) {
    commands.spawn((
        Camera,
        Camera3dBundle {
        transform: Transform::from_xyz(-5.0, 10.0, 0.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
        }
    ));
}