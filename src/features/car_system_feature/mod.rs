pub mod systems;
pub mod components;

use bevy::prelude::*;
use systems::spawn_car_system::spawn_car_system;
use systems::car_move_system::car_move_system;
use systems::car_acceleration_system::car_acceleration_system;
use systems::car_turn_system::car_turn_system;

pub struct CarFeature;

impl Plugin for CarFeature {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_car_system)
            .add_systems(Update, (
                car_acceleration_system,
                car_turn_system,
                car_move_system
            ));
    }
}