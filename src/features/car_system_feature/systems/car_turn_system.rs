use bevy::prelude::*;
use crate::features::car_system_feature::components::car_speed_component::CarSpeed;
use crate::features::car_system_feature::components::car_turn_component::CarTurn;

const TURN_SENSITIVITY: f32 = 10.0;
const TURN_BASE_POWER: f32 = 2.0;

pub fn car_turn_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut CarTurn, &CarSpeed)>
) {
    let right = keys.pressed(KeyCode::KeyD);
    let left = keys.pressed(KeyCode::KeyA);

    let target = if right { 1.0 } else { 0.0 } + if left { -1.0 } else { 0.0 };

    for (mut car_turn, car_speed) in &mut query {
        let turn_power = if car_speed.speed != 0.0 {
            TURN_BASE_POWER / car_speed.speed
        } else {
            0.0
        };
        car_turn.speed = car_turn.speed.lerp(
            target * turn_power, TURN_SENSITIVITY * time.delta_seconds()
        );
    }
}