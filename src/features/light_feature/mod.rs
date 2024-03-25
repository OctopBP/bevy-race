mod systems;

use bevy::prelude::*;
use systems::spawn_light_system::spawn_light_system;

pub struct LightFeature;

impl Plugin for LightFeature {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_light_system);
    }
}