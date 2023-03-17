use env_logger::Builder;
use log::LevelFilter;
use stainless_ffmpeg::probe::*;
use std::{collections::HashMap, env};

fn main() {
  let mut builder = Builder::from_default_env();
  builder.init();

  if let Some(path) = env::args().last() {
    let mut probe = DeepProbe::new(&path);
    let duration_params = CheckParameterValue {
      min: Some(40),
      max: Some(20000),
      num: None,
      den: None,
      th: None,
      pairs: None,
    };
    let black_duration_params = CheckParameterValue {
      min: Some(40),
      max: Some(20000),
      num: None,
      den: None,
      th: None,
      pairs: None,
    };
    let black_pixel_params = CheckParameterValue {
      min: None,
      max: None,
      num: None,
      den: None,
      th: Some(0.1),
      pairs: None,
    };
    let black_picture_params = CheckParameterValue {
      min: None,
      max: None,
      num: None,
      den: None,
      th: Some(0.98),
      pairs: None,
    };
    let spot_check = CheckParameterValue {
      min: None,
      max: Some(3),
      num: None,
      den: None,
      th: None,
      pairs: None,
    };
    let black_and_silence_check = CheckParameterValue {
      min: Some(40),
      max: None,
      num: None,
      den: None,
      th: None,
      pairs: None,
    };
    let mut audio_qualif = vec![];
    // definition : [Track::new(stream_index, channels_number)]
    // change this qualif based on the audio streams
    audio_qualif.push([Track::new(1, 1)].to_vec());
    audio_qualif.push([Track::new(2, 1)].to_vec());
    audio_qualif.push([Track::new(3, 8)].to_vec());
    audio_qualif.push([Track::new(4, 2)].to_vec());
    audio_qualif.push([Track::new(5, 2)].to_vec());
    audio_qualif.push([Track::new(6, 1), Track::new(7, 1)].to_vec());
    let loudness_check = CheckParameterValue {
      min: None,
      max: None,
      num: None,
      den: None,
      th: None,
      pairs: Some(audio_qualif),
    };
    let scene_check = CheckParameterValue {
      min: None,
      max: None,
      num: None,
      den: None,
      th: Some(10.0),
      pairs: None,
    };
    let ocr_check = CheckParameterValue {
      min: None,
      max: None,
      num: None,
      den: None,
      th: Some(14.0),
      pairs: None,
    };

    let mut silence_params = HashMap::new();
    let mut black_params = HashMap::new();
    let mut select_params = HashMap::new();
    let mut black_and_silence_params = HashMap::new();
    let mut scene_params = HashMap::new();
    let mut ocr_params = HashMap::new();
    let mut loudness_params = HashMap::new();
    silence_params.insert("duration".to_string(), duration_params);
    black_params.insert("duration".to_string(), black_duration_params);
    black_params.insert("picture".to_string(), black_picture_params);
    black_params.insert("pixel".to_string(), black_pixel_params);
    select_params.insert("spot_check".to_string(), spot_check);
    loudness_params.insert("pairing_list".to_string(), loudness_check);
    black_and_silence_params.insert("duration".to_string(), black_and_silence_check);
    scene_params.insert("threshold".to_string(), scene_check);
    ocr_params.insert("threshold".to_string(), ocr_check);
    let check = DeepProbeCheck {
      silence_detect: Some(silence_params),
      black_detect: Some(black_params),
      crop_detect: Some(select_params),
      black_and_silence_detect: Some(black_and_silence_params),
      scene_detect: Some(scene_params),
      ocr_detect: Some(ocr_params),
      loudness_detect: Some(loudness_params),
    };
    probe.process(LevelFilter::Off, check).unwrap();
    let result = serde_json::to_string(&probe).unwrap();
    println!("RESULT : \n{}\n", result);

    if let Some(result) = probe.result {
      println!("DEEP PROBE : \n{}", result);
    }
  }
}
