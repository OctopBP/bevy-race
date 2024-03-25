use bevy::prelude::*;
use crate::features::camera_feature::components::camera_target_component::CameraTarget;

const FOLLOW_LERP_SPEED: f32 = 3.0;

pub fn follow_target_system(
    time: Res<Time>,
    mut target_query: Query<&Transform, With<CameraTarget>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<CameraTarget>)>,
) {
    for target_transform in &mut target_query{
        for mut camera_transform in &mut camera_query {
            let forward = target_transform.forward();
            let offset = Vec3::new(forward.x * -1.0, 0.5, forward.z * -1.0);
            camera_transform.translation = camera_transform.translation.lerp(
                target_transform.translation + offset,
                time.delta_seconds() * FOLLOW_LERP_SPEED
            );
            camera_transform.rotation = camera_transform.looking_at(
                target_transform.translation, Vec3::Y
            ).rotation;
        }
    }
}