use bevy::prelude::*;
use crate::features::car_hud_feature::components::car_hud_speed_text_component::CarHudSpeedText;

pub fn create_car_hud_system(
    mut commands: Commands,
) {
    commands.spawn((
        CarHudSpeedText,
        TextBundle::from_sections([
            TextSection::from_style(
                TextStyle {
                    font_size: 40.0,
                    color: Color::GOLD,
                    ..default()
                }
            ),
            TextSection::new(
                "km/h",
                TextStyle {
                    font_size: 40.0,
                    ..default()
                },
            ),
        ])
        .with_text_justify(JustifyText::Right)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(10.0),
            ..default()
        }),
    ));
}

