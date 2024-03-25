mod systems;

use bevy::prelude::*;
use systems::build_track_system::build_track_system;

pub struct TrackFeature;

impl Plugin for TrackFeature {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_track_system);
    }
}