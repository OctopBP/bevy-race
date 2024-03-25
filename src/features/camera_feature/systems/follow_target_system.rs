use bevy::prelude::*;
use crate::features::camera_feature::components::camera_target_component::CameraTarget;

const FOLLOW_LERP_SPEED: f32 = 1.0;
const FOLLOW_OFFSET: Vec3 = Vec3::new(-5.0, 10.0, 0.0);

pub fn follow_target_system(
    time: Res<Time>,
    mut target_query: Query<&Transform, With<CameraTarget>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<CameraTarget>)>,
) {
    for target_transform in &mut target_query{
        for mut camera_transform in &mut camera_query {
            camera_transform.translation = camera_transform.translation.lerp(
                target_transform.translation + FOLLOW_OFFSET,
                time.delta_seconds() * FOLLOW_LERP_SPEED
            );
            camera_transform.rotation = camera_transform.looking_at(
                target_transform.translation, Vec3::Y
            ).rotation;
        }
    }
}