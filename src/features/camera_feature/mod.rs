mod systems;

use bevy::prelude::*;
use systems::spawn_camera_system::spawn_camera_system;

pub struct CameraFeature;

impl Plugin for CameraFeature {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera_system);
    }
}