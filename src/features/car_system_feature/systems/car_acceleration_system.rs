use bevy::prelude::*;
use crate::features::car_system_feature::components::car_speed_component::CarSpeed;

const MAX_SPEED: f32 = 3.0;
const BRAKES_ACCELERATION: f32 = 1.75;
const DRAG: f32 = 0.5;

pub fn car_acceleration_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut CarSpeed>
) {
    let speed_up = keys.pressed(KeyCode::KeyW);
    let breaks = keys.pressed(KeyCode::KeyS);

    for mut car_speed in &mut query {
        if breaks {
            car_speed.speed -= time.delta_seconds() * BRAKES_ACCELERATION;
        } else if speed_up {
            let acceleration = (car_speed.speed + 1.0).sqrt() - DRAG;
            car_speed.speed += time.delta_seconds() * acceleration;
        } else {
            car_speed.speed -= time.delta_seconds() * DRAG;
        }

        car_speed.speed = car_speed.speed.clamp(0.0, MAX_SPEED);
    }
}