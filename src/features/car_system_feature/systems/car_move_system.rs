use bevy::prelude::*;
use crate::features::car_system_feature::components::car_component::Car;
use crate::features::car_system_feature::components::car_speed_component::CarSpeed;

pub fn car_move_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &CarSpeed), With<Car>>
) {
    if keys.pressed(KeyCode::KeyW) {
        for (mut transform, speed) in &mut query {
            transform.translation.x += time.delta_seconds() * speed.speed;
        }
    }
}