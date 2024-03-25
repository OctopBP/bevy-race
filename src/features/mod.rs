mod car_system_feature;
mod camera_feature;
mod light_feature;
mod car_hud_feature;
mod track_feature;

use bevy::prelude::*;
use camera_feature::CameraFeature;
use car_system_feature::CarFeature;
use car_hud_feature::CarHudFeature;
use light_feature::LightFeature;
use track_feature::TrackFeature;

pub struct Features;

impl Plugin for Features {
    fn build(&self, app: &mut App) {
        app.add_plugins((TrackFeature, CarFeature, CarHudFeature, CameraFeature, LightFeature));
    }
}