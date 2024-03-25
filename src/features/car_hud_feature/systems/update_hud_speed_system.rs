use bevy::prelude::*;
use crate::features::car_hud_feature::components::car_hud_speed_text_component::CarHudSpeedText;
use crate::features::car_system_feature::components::car_component::Car;
use crate::features::car_system_feature::components::car_speed_component::CarSpeed;

pub fn update_hud_speed_system(
    car_speed_query: Query<&CarSpeed, With<Car>>,
    mut text_query: Query<&mut Text, With<CarHudSpeedText>>,
) {
    for car_speed in &car_speed_query {
        let speed = car_speed.speed * 100.0;
        for mut text in &mut text_query {
            text.sections[0].value = format!("{speed:.0}");
        }
    }
}