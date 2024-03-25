use bevy::prelude::*;
use crate::features::spawn_car_feature::components::car_component::Car;

pub fn spawn_car_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let race_gltf = asset_server.load("models/race.glb#Scene0");

    commands.spawn((
        Car,
        SceneBundle {
            scene: race_gltf,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }
    ));
}

