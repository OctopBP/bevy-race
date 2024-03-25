use bevy::prelude::*;
use crate::features::car_system_feature::components::car_component::Car;
use crate::features::car_system_feature::components::car_speed_component::CarSpeed;
use crate::features::car_system_feature::components::car_turn_component::CarTurn;

const TURN_STRENGTH: f32 = 0.5;

pub fn car_move_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &CarSpeed, &CarTurn), With<Car>>
) {
    for (mut transform, speed, turn) in &mut query {
        let forward = transform.forward();
        transform.translation += forward * time.delta_seconds() * speed.speed;
        transform.rotate(Quat::from_rotation_y(
            -turn.speed * speed.speed * time.delta_seconds() * TURN_STRENGTH)
        )
    }
}