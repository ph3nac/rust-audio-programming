use anyhow::Ok;
use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, FromSample, Sample, SizedSample};

fn main()->anyhow::Result<()>{
  // ホストのデフォルトデバイスを取得
  let host = cpal::default_host();  
  // デフォルトの出力デバイスを取得
  let device = host.default_output_device().unwrap();
  // デフォルトの出力設定を取得
  let config = device.default_output_config().unwrap();
  
  // サンプルフォーマットによって処理を分岐
    match config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        // cpal::SampleFormat::I24 => run::<I24>(&device, &config.into()),
        cpal::SampleFormat::I32 => run::<i32>(&device, &config.into()),
        // cpal::SampleFormat::I48 => run::<I48>(&device, &config.into()),
        cpal::SampleFormat::I64 => run::<i64>(&device, &config.into()),
        cpal::SampleFormat::U8 => run::<u8>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
        // cpal::SampleFormat::U24 => run::<U24>(&device, &config.into()),
        cpal::SampleFormat::U32 => run::<u32>(&device, &config.into()),
        // cpal::SampleFormat::U48 => run::<U48>(&device, &config.into()),
        cpal::SampleFormat::U64 => run::<u64>(&device, &config.into()),
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::F64 => run::<f64>(&device, &config.into()),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }
}

// sin波を生成して再生する
pub fn run<T>(device:&cpal::Device,config:&cpal::StreamConfig) -> Result<(),anyhow::Error> where T:SizedSample + FromSample<f32>,
{
  // sin波の計算を行うためにサンプルレートを取得しf32型に変換
    let sample_rate = config.sample_rate.0 as f32;
    // sin波を出力するチャンネル数を取得
    let channels = config.channels as usize;

    let mut sample_clock = 0f32;
    let mut next_value = move || {
      sample_clock = (sample_clock + 1.0) % sample_rate;
      (sample_clock * 440.0 * 2.0 * std::f32::consts::PI/sample_rate).sin()
    };
    
    let err_fn = |err| eprintln!("an error occurred on stream:{}",err);
    
    let stream = device.build_output_stream(config, move |data :&mut [T], _:&cpal::OutputCallbackInfo|{
      write_data(data,channels,&mut next_value)
    }, err_fn
    , None)?;

    stream.play()?;

    // 1秒間再生
    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

fn write_data<T>(output:&mut[T],channels:usize,next_sample:&mut dyn FnMut()->f32) where T:Sample+FromSample<f32>
{
  // 出力データのチャンネルごとに現在のサンプルのオーディオバッファを埋める
  for frame in output.chunks_mut(channels){
    //  サンプルを生成
    let value:T = T::from_sample_(next_sample());
    // チャンネルごとにサンプルを埋める(オーディオバッファに書き込む)
    for sample in frame.iter_mut(){
      *sample = value;
    }
  }
}
