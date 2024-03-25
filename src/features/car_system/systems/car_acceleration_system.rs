use bevy::prelude::*;
use crate::features::car_system::components::car_speed_component::CarSpeed;

const MAX_SPEED: f32 = 30.0;
const ACCELERATION: f32 = 5.0;
const BRAKES_ACCELERATION: f32 = 10.0;

pub fn car_acceleration_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut CarSpeed>
) {
    let speed_up = keys.pressed(KeyCode::KeyW);

    for mut car_speed in &mut query {
        if speed_up {
            car_speed.speed += time.delta_seconds() * ACCELERATION;
        } else {
            car_speed.speed -= time.delta_seconds() * BRAKES_ACCELERATION;
        }

        car_speed.speed = car_speed.speed.clamp(0.0, MAX_SPEED);
    }
}