use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{StreamConfig, SampleFormat, SampleRate};
use std::f32::consts::PI;
use std::sync::mpsc::channel;
use std::time::Duration;
use num_complex::{Complex, ComplexFloat};

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No default output device available");
    let config = device.default_output_config().unwrap();


    let mut supported_formats_range = config.sample_format();

    let sample_rate = supported_formats_range.sample_size() as f32;

    // サンプルレートとサイン波の周波数
    let sample_rate = 44100.0; // 44.1kHz
    let frequency = 440.0; // 440Hz
    let amplitude = 0.5; // 音量の振幅

    // サイン波生成のためのステップ
    let mut phase = 0.0;
    let phase_increment = 2.0 * PI * frequency / sample_rate;

    // ストリームの設定
    let config = StreamConfig {
        channels: 1,
        sample_rate: SampleRate(sample_rate as u32),
        buffer_size: cpal::BufferSize::Default,
    };

    let mut phase = 0.0;

    let mut stream = device.build_output_stream(
        &config,
        move |output: &mut [f32], _| {
            for sample in output.iter_mut() {
                let sample_value = amplitude * (phase.sin());
                *sample = sample_value;
                phase += phase_increment;
                if phase > 2.0 * PI {
                    phase -= 2.0 * PI;
                }
            }
        },
        move |err| {
            eprintln!("Error occurred: {:?}", err);
        },
        None,
    ).unwrap();

    stream.play().unwrap();
    std::thread::sleep(Duration::from_secs(2)); // 2秒間再生
}
