use std::{env, ffi::OsStr, fs::{self, read_dir}, ops::Deref, path::PathBuf, process::exit};

use clap::Parser;
use pcap::{Capture, Device, Packet, PacketCodec};
use ratatui::crossterm::style::Stylize;
use trigerror::{bytes_to_u16, cli::CLI, packet::Codec, ring_buffer::{self, RingBuffer}, trigerror::Trigerror};

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

  let mut packets = vec![];
  let mut savefile = capture.savefile("single_packet.pcap").unwrap();
  let packet = capture.next_packet().unwrap();
  packets.push(packet.clone());
  savefile.write(&packet);
  // for byte in packet.data
  // { print!("{:x} ", byte); }
  // println!();
  println!();
  let dest_mac = (packet.data[0], packet.data[1], packet.data[2], packet.data[3], packet.data[4], packet.data[5]);
  let src_mac = (packet.data[6], packet.data[7], packet.data[8], packet.data[9], packet.data[10], packet.data[11]);
  let ether_type = bytes_to_u16(packet.data[12], packet.data[13]);
  println!(
    "destination MAC address: {:02x?}-{:02x?}-{:02x?}-{:02x?}-{:02x?}-{:02x?}",
    dest_mac.0, dest_mac.1, dest_mac.2, dest_mac.3, dest_mac.4, dest_mac.5
  );
  println!(
    "source MAC address: {:02x?}-{:02x?}-{:02x?}-{:02x?}-{:02x?}-{:02x?}",
    src_mac.0, src_mac.1, src_mac.2, src_mac.3, src_mac.4, src_mac.5
  );
  println!("ether type: 0x{:04x?}", ether_type);
  print!("Protocol: ");
  match ether_type
  {
    0x0800 => println!("IPv4"),
    0x0804 => println!("Chaosnet"),
    0x0806 => println!("ARP"),
    0x0842 => println!("Wake-on-LAN"),
    0x22ea => println!("Stream Reservation Protocol"),
    0x22f0 => println!("AVTP"),
    0x22f3 => println!("IETF TRILL Protocol"),
    0x6002 => println!("DEC MOP RC"),
    0x6003 => println!("DECnet Phase IV, DNA Routing"),
    0x6004 => println!("DEC LAT"),
    0x8035 => println!("RARP"),
    0x809b => println!("ApplteTalk"),
    0x80d5 => println!("LLC PDU"),
    0x80f3 => println!("AARP"),
    0x8100 => println!("VLAN"),
    0x8102 => println!("SLPP"),
    0x8103 => println!("VLACP"),
    0x8137 => println!("IPX"),
    0x8204 => println!("QNX Qnet"),
    0x86dd => println!("IPv6"),
    0x8808 => println!("Ethernet flow control"),
    0x8809 => println!("LACP"),
    0x8819 => println!("CobraNet"),
    0x8847 => println!("MPLS unicast"),
    0x8848 => println!("MPLS multicast"),
    0x8863 => println!("PPPoE Discovery Stage"),
    0x8864 => println!("PPPoE Session Stage"),
    0x887b => println!("HomePlug"),
    0x888e => println!("EAP over LAN"),
    0x8892 => println!("PROFINET"),
    0x889a => println!("HyperSCSI"),
    0x88a2 => println!("ATA over Ethernet"),
    0x88a4 => println!("EtherCAT"),
    0x88a8 => println!("Service VLAN tag identifier"),
    0x88ab => println!("Ethernet Powerlink"),
    0x88b8 => println!("GOOSE"),
    0x88b9 => println!("GSE"),
    0x88ba => println!("SV"),
    0x88bf => println!("MikroTik RoMON"),
    0x88cc => println!("LLDP"),
    0x88cd => println!("SERCOS III"),
    0x88e1 => println!("HomePlug Green PHY"),
    0x88e3 => println!("Media Redundancy Protocol"),
    0x88e5 => println!("MACsec"),
    0x88e7 => println!("PBB"),
    0x88f7 => println!("PTP (this is our thingy)"),
    0x88f8 => println!("NC-SI"),
    0x88fb => println!("PRP"),
    0x8902 => println!("CFM"),
    0x8906 => println!("FCoE"),
    0x8914 => println!("FCoE initialization protocol"),
    0x8915 => println!("RoCE"),
    0x891d => println!("TTE"),
    0x893a => println!("1905.1 IEEE Protocol"),
    0x892f => println!("HSR"),
    0x9000 => println!("Ethernet Configuration Testing Protocol"),
    0x9100 | 0x9200 => println!("Service VLAN tag identifier (S-Tag) on Q-in-Q tunnel"),
    0xf1c1 => println!("Redundancy Tag"),
    _ => println!("unknown protocol")
  }

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

  // // Another approach
  // let mut count = 0;
  // let mut ring_buffer = RingBuffer::new();
  // ring_buffer.count_before = 50;
  // for packet in capture.iter(Codec)
  // {
  //   let packet = packet.unwrap();
  //   ring_buffer.push(packet);
  //   count += 1;
  //   if count >= 100 { break; }
  // }
  // for packet in ring_buffer.drain()
  // { println!("{:?}", packet.header); }

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
