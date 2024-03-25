mod systems;
pub mod components;

use bevy::prelude::*;
use systems::spawn_camera_system::spawn_camera_system;
use systems::follow_target_system::follow_target_system;

pub struct CameraFeature;

impl Plugin for CameraFeature {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera_system)
            .add_systems(Update, follow_target_system);
    }
}