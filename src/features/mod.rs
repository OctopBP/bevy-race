mod car_system;
mod camera_feature;
mod light_feature;

use bevy::prelude::*;
use camera_feature::CameraFeature;
use car_system::CarFeature;
use crate::features::light_feature::LightFeature;

pub struct Features;

impl Plugin for Features {
    fn build(&self, app: &mut App) {
        app.add_plugins((CarFeature, CameraFeature, LightFeature));
    }
}