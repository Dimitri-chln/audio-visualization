use bevy::prelude::*;

use crate::{
    resources::{SoundBuffer, SoundStream},
    systems,
};

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .init_non_send_resource::<SoundStream>()
            .init_resource::<SoundBuffer>()
            .add_systems(Startup, systems::setup)
            .add_systems(
                Update,
                (systems::receive_sound_samples, systems::update_soundbars),
            );
    }
}
