mod features;

use bevy::prelude::*;
use crate::features::Features;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Features))
        .run();
}