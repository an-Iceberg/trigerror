use std::{env, ffi::OsStr, fs::{self, File, read_dir}, io::{BufWriter, Write}, ops::Deref, path::PathBuf, process::exit, time::{Duration, SystemTime}};

use chrono::{DateTime, Utc};
use clap::Parser;
use pcap::{Capture, Device, Packet, PacketCodec};
use pcap_file::pcap::{PcapPacket, PcapWriter};
use ratatui::crossterm::style::Stylize;
use trigerror::{bytes_to_u16, cli::CLI, get_ether_type, packet::Codec, ring_buffer::{self, RingBuffer}, timeval_to_i64, trigerror::Trigerror};

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

  // TODO: move these into trigerror mod.
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
      // TODO: adjust these parameters
      cap.promisc(true)
        .immediate_mode(true)
        // .snaplen(5_000)
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

  if let Some(filters) = trigerror.filters
  {
    match capture.filter(filters.as_str(), true)
    {
      Ok(_) => println!("[ {} ] filters set and compiled", "OK".green()),
      Err(error) =>
      {
        println!("[ {} ] couldn't set filters b/c: {}", "ERROR".red(), error);
        dbg!{filters};
        exit(-1);
      }
    }
  }
  else { println!("[ {} ] no filters set; recording everything", "OK".green()); }

  // let mut packets = vec![];
  // let mut savefile = capture.savefile("single_packet.pcap").unwrap();
  // let packet = capture.next_packet().unwrap();
  // packets.push(packet.clone());
  // savefile.write(&packet);
  // println!();
  // let dest_mac = (packet.data[0], packet.data[1], packet.data[2], packet.data[3], packet.data[4], packet.data[5]);
  // let src_mac = (packet.data[6], packet.data[7], packet.data[8], packet.data[9], packet.data[10], packet.data[11]);
  // let ether_type = bytes_to_u16(packet.data[12], packet.data[13]);
  // println!(
  //   "destination MAC address: {:02x?}-{:02x?}-{:02x?}-{:02x?}-{:02x?}-{:02x?}",
  //   dest_mac.0, dest_mac.1, dest_mac.2, dest_mac.3, dest_mac.4, dest_mac.5
  // );
  // println!(
  //   "source MAC address: {:02x?}-{:02x?}-{:02x?}-{:02x?}-{:02x?}-{:02x?}",
  //   src_mac.0, src_mac.1, src_mac.2, src_mac.3, src_mac.4, src_mac.5
  // );
  // println!("ether type: 0x{:04x?}", ether_type);
  // print!("Protocol: ");

  // %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

  // print!("First approach: ");
  // let mut savefile = capture.savefile("test.pcap").unwrap();
  // for _ in 0..50
  // {
  //   savefile.write(&capture.next_packet().unwrap());
  //   print!(".");
  // }
  // println!(" Done.");

  // print!("Second approach: ");
  // let mut savefile = capture.savefile("test2.pcap").unwrap();
  // capture.for_each(Some(50), |packet|
  // {
  //   savefile.write(&packet);
  //   print!(".")
  // }).unwrap();
  // println!(" Done.");

  // %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

  // // Another approach
  // let mut triggered = false;
  // let mut count = 0;
  // let mut pre_buffer = RingBuffer::new(20, trigerror.time_before);
  // let mut post_buffer: Vec<Packet> = vec![];
  // for packet in capture.iter(Codec)
  // {
  //   let packet = packet.unwrap();

  //   if !triggered
  //   {
  //     pre_buffer.push(packet);
  //   }
  //   else
  //   {
  //     post_buffer.push(packet);
  //   }

  //   count += 1;
  //   if count >= 50 { triggered = true; }
  //   if count >= 100 { break; }
  // }
  // println!("pre_buffer ({}):", pre_buffer.len());
  // for packet in pre_buffer.drain()
  // {
  //   print!("{}, ", get_ether_type(bytes_to_u16(packet.data[12], packet.data[13])));
  // }
  // println!();
  // println!("post_buffer ({}):", post_buffer.len());
  // for packet in &post_buffer
  // {
  //   print!("{}, ", get_ether_type(bytes_to_u16(packet.data[12], packet.data[13])));
  // }
  // println!();

  // let system_time = SystemTime::now();
  // let datetime: DateTime<Utc> = system_time.into();
  // let time_string = datetime.format("%Y-%m-%d_%H:%M:%S").to_string();
  // let first_interface = trigerror.interfaces.first().unwrap();

  // let outfile = File::create(format!("trigerror_{first_interface}_{time_string}.pcap")).expect("couldn't create file");
  // let mut pcap_writer = PcapWriter::new(outfile).expect("Error writing file");
  // // Writing the pre_buffer to the file
  // for packet in pre_buffer.drain()
  // { pcap_writer.write_packet(&packet.to_pcap_packet()).unwrap(); }
  // // Writing the post_buffer to the file
  // for packet in post_buffer
  // { pcap_writer.write_packet(&packet.to_pcap_packet()).unwrap(); }

  // %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

  let mut counter = 0;
  let mut buffer = vec![];

  let system_time = SystemTime::now();
  let datetime: DateTime<Utc> = system_time.into();
  // This is approximately when the capturing has started.
  // TODO: this should be the timestamp when the first trigger happens.
  let time_string = datetime.format("%Y-%m-%d_%H:%M:%S").to_string();

  while let Ok(packet) = capture.next_packet()
  {
    let packet = PcapPacket::new_owned(
      // NOTE: this needs confirmation that it's correct.
      Duration::from_secs_f64(timeval_to_i64(packet.header.ts)),
      packet.header.caplen,
      packet.data.into()
    );
    print!("{}, ", get_ether_type(bytes_to_u16(packet.data[12], packet.data[13])));
    buffer.push(packet);
    counter += 1;
    if counter > 50 { break; }
  }

  let first_interface = trigerror.interfaces.first().unwrap();

  let outfile = File::create(format!("trigerror_{first_interface}_{time_string}.pcap")).expect("couldn't create file");
  let mut pcap_writer = PcapWriter::new(outfile).expect("Error writing file");
  for packet in buffer
  { pcap_writer.write_packet(&packet).unwrap(); }

  // TODO: once one file is done, don't exit program, but listen to next error.

  // %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

  // let mut writer = BufWriter::new(&outfile);

  // println!();
  // // Write single packet to console.
  // for byte in &pre_buffer.drain().first().unwrap().data
  // { print!("{:02x} ", byte); }
  // // Write pre buffer to file.
  // for packet in pre_buffer.drain()
  // { writer.write_all(&packet.data).unwrap(); }
  // // Write post buffer to file
  // for packet in &post_buffer
  // { writer.write_all(&packet.data).unwrap(); }
  // println!();
  // println!();
  // // Write last packet to console.
  // for byte in &post_buffer.last().unwrap().data
  // { print!("{:02x} ", byte); }

  // %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

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
