extern crate env_logger;
extern crate log;
extern crate stainless_ffmpeg;
extern crate serde_json;

use env_logger::{Builder, Env};
use std::env;
use stainless_ffmpeg::probe::*;
use log::LevelFilter;

fn main() {
  Builder::from_env(Env::default().default_filter_or("debug")).init();

  if let Some(path) = env::args().last() {
    let mut probe = Probe::new(&path);
    probe.process(LevelFilter::Off).unwrap();
    let result = serde_json::to_string(&probe).unwrap();
    println!("{}", result);

    if let Some(format) = probe.format {
      println!("Format : \n{}", format);
    }
  }
}