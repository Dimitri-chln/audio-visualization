use bevy::prelude::*;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    StreamConfig,
};
use ringbuf::HeapRb;
use tokio::sync::mpsc;

use crate::constants::SOUND_BUFFER_SIZE;

pub struct SoundStream {
    pub _stream: cpal::platform::Stream,
    pub receiver: mpsc::Receiver<Vec<f32>>,
}

impl Default for SoundStream {
    fn default() -> Self {
        let host = cpal::default_host();
        let output_device = host.default_output_device().unwrap();
        let config = StreamConfig::from(output_device.default_output_config().unwrap());

        let (sender, receiver) = mpsc::channel(1);

        // Build the stream
        let input_stream = output_device
            .build_input_stream(
                &config,
                move |data: &[f32], _| {
                    if let Err(error) = sender.blocking_send(data.to_vec()) {
                        eprintln!("an error occurred while sending data: {error}");
                    }
                },
                |error| eprintln!("an error occurred on stream: {}", error),
                None,
            )
            .unwrap();

        // Play the stream
        input_stream.play().unwrap();

        Self {
            _stream: input_stream,
            receiver,
        }
    }
}

#[derive(Resource)]
pub struct SoundBuffer(pub HeapRb<Vec<f32>>);

impl Default for SoundBuffer {
    fn default() -> Self {
        Self(HeapRb::new(SOUND_BUFFER_SIZE))
    }
}
