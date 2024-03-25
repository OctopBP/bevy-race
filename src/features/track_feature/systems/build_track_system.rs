use bevy::math::Vec3;
use bevy::prelude::*;

pub fn build_track_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let gltf_model = asset_server.load("models/melbourne-track.glb#Scene0");
    commands.spawn(SceneBundle {
        scene: gltf_model,
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::X, Vec3::Y),
        ..default()
    });
}