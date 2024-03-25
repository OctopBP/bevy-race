mod spawn_car_feature;
mod camera_feature;
mod light_feature;

use bevy::prelude::*;
use camera_feature::CameraFeature;
use spawn_car_feature::SpawnCarFeature;
use crate::features::light_feature::LightFeature;

pub struct Features;

impl Plugin for Features {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraFeature, LightFeature, SpawnCarFeature));
    }
}