use std::{env, ffi::OsStr, fs::{self, read_dir}, process::exit};

use clap::Parser;
use pcap::{Capture, Device, Packet, PacketCodec};
use trigerror::{cli::CLI, configure_trigerror_from_cli, extract_config_from_ini, owned_packet::Codec, trigerror::Trigerror};

fn main()
{
  // Check if `trigerror.ini` file exists in the `cwd`.
  let cwd = env::current_dir().expect("call to cwd failed");
  // dbg!{cwd.display()};
  // let paths = fs::read_dir(cwd).unwrap();

  // Construct trigerror instance with configuration.
  let mut trigerror = if read_dir(cwd)
    .expect("couldn't read cwd")
    .flatten()
    .map(|path| path.file_name())
    .any(|file| file == OsStr::new("trigerror.ini"))
  { extract_config_from_ini() }
  else
  { Trigerror::new() };
  // dbg!{trigerror};

  // Read CLI arguments and reconfigure if necessary
  trigerror = configure_trigerror_from_cli(CLI::parse(), trigerror);

  dbg!{trigerror};
  // exit(0);

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
    .immediate_mode(true) // This has a higher load on CPU.
    // .snaplen(5_000)
    .open()
    .unwrap();

  // One approach
  let mut savefile = capture.savefile("test.pcap").unwrap();
  for _ in 0..50
  {
    let packet = capture.next_packet().unwrap();
    savefile.write(&packet);
  }

  // // Another approach
  // for _ in 0..50
  // { capture.next_packet().ok(); }
  // for packet in capture.iter(Codec)
  // {
  //   let packet = packet.unwrap();
  //   dbg!{packet};
  // }

  // Alternative approach
  let mut savefile = capture.savefile("test3.pcap").unwrap();
  capture.for_each(Some(50), |packet| savefile.write(&packet)).unwrap();

  // for packet in packets { savefile.write(&packet); }

  // while let Ok(packet) = capture.next_packet()
  // { println!("{:?}", packet); }

  // let packet = capture.next_packet();
  // // dbg!{packet};
  // match packet
  // {
  //   Ok(packet) =>
  //   {
  //     dbg!{&packet};
  //     println!();
  //     dbg!{&packet.header};
  //   },
  //   Err(error) => { dbg!{error}; },
  // };

  // if let Ok(packet) = capture.next_packet()
  // {
  //   dbg!{packet};
  // }
}
