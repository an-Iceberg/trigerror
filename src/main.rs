use std::{env, ffi::OsStr, fs};

use clap::Parser;
use pcap::{Capture, Device};
use trigerror::{cli::CLI, extract_config, trigerror::Trigerror};
use ini::ini;

fn main()
{
  // Check if `trigerror.ini` file exists in the `cwd`.
  let cwd = env::current_dir().unwrap();
  // dbg!{cwd.display()};
  let paths = fs::read_dir(cwd).unwrap();

  // Construct trigerror instance with configuration.
  let trigerror = if paths
    .flatten()
    .map(|path| path.file_name())
    .any(|file| file == OsStr::new("trigerror.ini"))
  { extract_config() }
  else
  { Trigerror::new() };
  dbg!{trigerror};

  // Read CLI arguments
  // let cli = CLI::parse();
  // println!("interface(s): {:?}", cli.interfaces);

  // Reconfigure if necessary

  // Listen on interfaces.

  dbg!{Device::list().unwrap()};
  dbg!{"{:?}", Device::list()
    .unwrap()
    .iter()
    .map(|device| device.clone().name)
    .collect::<Vec<String>>()
  };

  let ethernet = Device::list()
    .unwrap()
    .iter()
    .find(|device| device.name.starts_with("enp"))
    .unwrap()
    .to_owned();

  let mut capture = Capture::from_device(ethernet)
    .unwrap()
    .promisc(true)
    .snaplen(5_000)
    .open()
    .unwrap();

  let packet = capture.next_packet();
  // dbg!{packet};
  match packet
  {
    Ok(packet) => { dbg!{packet}; },
    Err(error) => { dbg!{error}; },
  };
  // if let Ok(packet) = capture.next_packet()
  // {
  //   dbg!{packet};
  // }
}
