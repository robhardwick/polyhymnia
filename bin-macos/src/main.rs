mod error;

use std::env;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Sample, SampleFormat,
};

use libpoly::Poly;

use error::Error;

pub fn main() -> Result<(), Error> {
    pretty_env_logger::init();

    let seed = env::args()
        .nth(1)
        .and_then(|arg| arg.parse::<u64>().ok())
        .or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .ok()
                .map(|duration| duration.as_secs())
        })
        .ok_or(Error::Seed)?;

    let host = cpal::default_host();
    let device = host.default_output_device().ok_or(Error::NoDefaultDevice)?;
    let config = device.default_output_config()?;

    match config.sample_format() {
        SampleFormat::I16 => run::<i16>(seed, &device, &config.into()),
        SampleFormat::U16 => run::<u16>(seed, &device, &config.into()),
        SampleFormat::F32 => run::<f32>(seed, &device, &config.into()),
    }
}

pub fn run<T>(seed: u64, device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), Error>
where
    T: Sample + std::fmt::Debug,
{
    let mut poly = Poly::new(seed, config.sample_rate.0)?;

    let channels = config.channels as usize;
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let value: T = cpal::Sample::from::<f32>(&poly.next());
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        |err| eprintln!("an error occurred on stream: {}", err),
    )?;

    stream.play()?;

    loop {
        thread::park()
    }
}
