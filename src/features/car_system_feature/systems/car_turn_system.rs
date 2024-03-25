use bevy::prelude::*;
use crate::features::car_system_feature::components::car_turn_component::CarTurn;

const TURN_SENSITIVITY: f32 = 5.0;

pub fn car_turn_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut CarTurn>
) {
    let right = keys.pressed(KeyCode::KeyD);
    let left = keys.pressed(KeyCode::KeyA);
    let target = if right { 1.0 } else { 0.0 } + if left { -1.0 } else { 0.0 };

    for mut car_turn in &mut query {
        car_turn.speed = car_turn.speed.lerp(target, TURN_SENSITIVITY * time.delta_seconds());
    }
}