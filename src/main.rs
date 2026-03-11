use std::{env, ffi::OsStr, fs::{self, read_dir}, ops::Deref, process::exit};

use clap::Parser;
use pcap::{Capture, Device, Packet, PacketCodec};
use trigerror::{cli::CLI, configure_trigerror_from_cli, packet::Codec, extract_config_from_ini, ring_buffer::{self, RingBuffer}, trigerror::Trigerror};

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

  dbg!{&trigerror};
  // exit(0);

  // Listen on interfaces.

  dbg!{Device::list().unwrap()};
  dbg!{"{:?}", Device::list()
    .unwrap()
    .iter()
    .map(|device| device.clone().name)
    .collect::<Vec<String>>()
  };

  let first_device = Device::list()
    .unwrap()
    .iter()
    .find(|device| device.name.contains(trigerror.interfaces.first().unwrap()))
    .unwrap()
    .to_owned();

  let mut capture = Capture::from_device(first_device)
    .unwrap()
    .promisc(true)
    .immediate_mode(true) // This has a higher load on CPU.
    // .snaplen(5_000)
    .open()
    .unwrap();

  // This is how one would set the filter(s).
  // capture.filter("ptp and gptp", true).ok();

  print!("First approach: ");
  // One approach
  let mut savefile = capture.savefile("test.pcap").unwrap();
  for _ in 0..50
  {
    savefile.write(&capture.next_packet().unwrap());
    print!(".");
  }
  println!(" Done.");

  print!("Second approach: ");
  // Alternative approach
  let mut savefile = capture.savefile("test2.pcap").unwrap();
  capture.for_each(Some(50), |packet|
  {
    savefile.write(&packet);
    print!(".")
  }).unwrap();
  println!(" Done.");

  // Another approach
  let mut count = 0;
  let mut ring_buffer = RingBuffer::new();
  ring_buffer.count_before = 50;
  for packet in capture.iter(Codec)
  {
    let packet = packet.unwrap();
    ring_buffer.push(packet);
    count += 1;
    if count >= 100 { break; }
  }
  for packet in ring_buffer.drain()
  { println!("{:?}", packet.header); }

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
