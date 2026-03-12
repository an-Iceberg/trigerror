use std::{env, ffi::OsStr, fs::{self, read_dir}, ops::Deref, path::PathBuf, process::exit};

use clap::Parser;
use pcap::{Capture, Device, Packet, PacketCodec};
use ratatui::crossterm::style::Stylize;
use trigerror::{cli::CLI, packet::Codec, ring_buffer::{self, RingBuffer}, trigerror::Trigerror};

fn main()
{
  // Check if `trigerror.ini` file exists in the `cwd`.
  // let cwd = match env::current_dir()
  // {
  //   Ok(cwd) =>
  //   {
  //     println!("[ {} ] got cwd", "OK".green());
  //     cwd
  //   }
  //   Err(error) =>
  //   {
  //     println!("[ {} ] couldn't get cwd b/c: {}", "ERROR".red(), error);
  //     exit(-1);
  //   }
  // };

  // let read_dir = match read_dir(cwd)
  // {
  //   Ok(read_dir) =>
  //   {
  //     println!("[ {} ] read directory", "OK".green());
  //     read_dir
  //   }
  //   Err(error) =>
  //   {
  //     println!("[ {} ] couldn't read directory b/c: {}", "ERROR".red(), error);
  //     exit(-1);
  //   }
  // };

  // let config_file_present = read_dir
  //   .flatten()
  //   .map(|path| path.file_name())
  //   .any(|file| file == OsStr::new("trigerror.ini"));

  let mut trigerror = Trigerror::configure_from_ini(PathBuf::from("trigerror.ini"));
  trigerror.configure_from_cli(CLI::parse());

  let devices = match Device::list()
  {
    Ok(devs) =>
    {
      println!("[ {} ] listed devices", "OK".green());
      devs
    }
    Err(error) =>
    {
      println!("[ {} ] couldn't list devices b/c: {}", "ERROR".red(), error);
      exit(-1);
    }
  };

  let first_device = match devices
    .iter()
    .find(|device| device.name.contains(trigerror.interfaces.first().unwrap()))
  {
    Some(first_device) =>
    {
      println!("[ {} ] device {} found", "OK".green(), first_device.name);
      first_device.to_owned()
    }
    None =>
    {
      println!(
        "[ {} ] device {} not found in device list. Device list: {:?}",
        "ERROR".red(),
        trigerror.interfaces.first().unwrap(),
        devices.iter().map(|device| device.name.to_owned()).collect::<Vec<String>>(),
      );
      exit(-1);
    }
  };

  let capture_inactive = match Capture::from_device(first_device)
  {
    Ok(cap) =>
    {
      println!("[ {} ] created capture device", "OK".green());
      cap.promisc(true)
        .immediate_mode(true)
        .snaplen(5_000)
    }
    Err(error) =>
    {
      println!("[ {} ] couldn't create capture device b/c: {}", "ERROR".red(), error);
      exit(-1);
    }
  };

  let mut capture = match capture_inactive.open()
  {
    Ok(cap) =>
    {
      println!("[ {} ] opened capture device", "OK".green());
      cap
    }
    Err(error) =>
    {
      println!("[ {} ] couldn't open capture device b/c: {}", "ERROR".red(), error);
      exit(-1);
    }
  };

  // let mut capture = Capture::from_device(first_device)
  //   .unwrap()
  //   .promisc(true)
  //   .immediate_mode(true) // This has a higher load on CPU.
  //   // .snaplen(5_000)
  //   .open()
  //   .unwrap();

  // This is how one would set the filter(s).
  // capture.filter("ptp and gptp", true).ok();

  print!("First approach: ");
  let mut savefile = capture.savefile("test.pcap").unwrap();
  for _ in 0..50
  {
    savefile.write(&capture.next_packet().unwrap());
    print!(".");
  }
  println!(" Done.");

  print!("Second approach: ");
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
