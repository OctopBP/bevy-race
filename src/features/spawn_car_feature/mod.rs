pub mod systems;
pub mod components;

use bevy::prelude::*;
use systems::spawn_car_system::spawn_car_system;

pub struct SpawnCarFeature;

impl Plugin for SpawnCarFeature {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_car_system);
    }
}