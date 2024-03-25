use bevy::prelude::*;

pub fn spawn_light_system(
    mut commands: Commands,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 50000.,
            ..default()
        },
        ..default()
    });
}