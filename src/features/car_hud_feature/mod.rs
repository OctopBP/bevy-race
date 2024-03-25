pub mod systems;
pub mod components;

use bevy::prelude::*;
use systems::create_car_hud_system::create_car_hud_system;
use systems::update_hud_speed_system::update_hud_speed_system;

pub struct CarHudFeature;

impl Plugin for CarHudFeature {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, create_car_hud_system)
            .add_systems(Update, update_hud_speed_system);
    }
}