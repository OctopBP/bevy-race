use std::f32::consts::PI;
use bevy::prelude::*;

pub fn spawn_light_system(
    mut commands: Commands,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 5000.0,
            color: Color::Rgba { red: 1.0, green: 0.95, blue: 0.9, alpha: 1.0 },
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
}