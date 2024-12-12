use std::{collections::HashSet, hash::RandomState};

use bevy::{prelude::*, window::PrimaryWindow};
use ringbuf::traits::{Consumer, RingBuffer};

use crate::{
    components::{Index, SoundBar},
    constants::AMPLITUDE_SCALE,
    resources::{SoundBuffer, SoundStream},
    utils::{merge::Merge, vec_zip::VecZip},
};

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn receive_sound_samples(
    mut sound_stream: NonSendMut<SoundStream>,
    mut sound_buffer: ResMut<SoundBuffer>,
) {
    if let Ok(samples) = sound_stream.receiver.try_recv() {
        sound_buffer.0.push_overwrite(samples);
    }
}

pub fn update_soundbars(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    sound_buffer: Res<SoundBuffer>,
    mut sound_bars: Query<(&Index, &mut Sprite, &mut Transform, Entity), With<SoundBar>>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };

    let samples = VecZip::new(sound_buffer.0.as_slices().merge());
    let values = samples
        .map(|samples| {
            let (count, sum) = samples
                .into_iter()
                .fold((0_usize, 0.), |(count, sum), value| {
                    (count + 1, sum + value.unwrap_or(0.))
                });

            sum / count as f32
        })
        .collect::<Vec<_>>();

    let mut indexes = (0..values.len()).collect::<HashSet<_, RandomState>>();
    let bar_width = window.width() / values.len() as f32;
    let x_translation = |i: usize| bar_width * (i as f32 + 0.5) - window.width() / 2.;
    let size = |sample: f32| {
        Vec2::new(
            bar_width,
            2. * sample.abs() * window.height() * AMPLITUDE_SCALE,
        )
    };

    for (index, mut sprite, mut transform, entity) in &mut sound_bars {
        let i = index.0;
        indexes.remove(&i);

        if i >= values.len() {
            commands.entity(entity).despawn();
            continue;
        }

        transform.translation.x = x_translation(i);

        let new_size = size(values[i]);
        sprite.custom_size = Some(
            sprite
                .custom_size
                .map_or(new_size, |size| size.lerp(new_size, 0.5)),
        );
    }

    for i in indexes {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(size(values[i])),
                    color: Color::WHITE,
                    ..default()
                },
                transform: Transform::from_xyz(x_translation(i), 0., 0.),
                ..default()
            })
            .insert(SoundBar)
            .insert(Index(i));
    }
}
