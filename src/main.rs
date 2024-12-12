use bevy::prelude::*;

use audio_visualization::MainPlugin;

fn main() {
    App::new().add_plugins(MainPlugin).run();
}
