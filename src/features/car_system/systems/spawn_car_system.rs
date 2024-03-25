use bevy::prelude::*;
use crate::features::camera_feature::components::camera_target_component::CameraTarget;
use crate::features::car_system::components::car_component::Car;
use crate::features::car_system::components::car_speed_component::CarSpeed;

pub fn spawn_car_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let race_gltf = asset_server.load("models/race.glb#Scene0");

    commands.spawn((
        Car,
        CarSpeed { speed: 0.0 },
        CameraTarget,
        SceneBundle {
            scene: race_gltf,
            transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::X, Vec3::Y),
            ..default()
        }
    ));
}

