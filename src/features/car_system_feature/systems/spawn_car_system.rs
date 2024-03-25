use bevy::prelude::*;
use crate::features::camera_feature::components::camera_target_component::CameraTarget;
use crate::features::car_system_feature::components::car_component::Car;
use crate::features::car_system_feature::components::car_speed_component::CarSpeed;
use crate::features::car_system_feature::components::car_turn_component::CarTurn;

pub fn spawn_car_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let race_gltf = asset_server.load("models/raceCarRed.glb#Scene0");

    commands.spawn((
        Car,
        CarSpeed { speed: 0.0 },
        CarTurn { speed: 0.0 },
        CameraTarget,
        SceneBundle {
            scene: race_gltf,
            transform: Transform::IDENTITY
                .looking_at(Vec3::X, Vec3::Y)
                .with_scale(Vec3::ONE * 0.25),
            ..default()
        }
    ));
}

