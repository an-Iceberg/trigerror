#![allow(clippy::needless_return)]
#![allow(mixed_script_confusables)]

pub mod cli;
pub mod config;
pub mod constants;
pub mod eth_frame;
pub mod gptp;
pub mod packet;
pub mod protocol;
pub mod recording;
pub mod ring_buffer;
pub mod protocols;
pub mod writer;

use std::{process::exit, time::{Duration, SystemTime}};
use chrono::{DateTime, Utc};
use libc::timeval;
use pcap::{Active, Capture, Device, Packet};
use pcap_file::pcap::PcapPacket;
use colored::Colorize;
use crate::config::Config;

// TODO: move this to protocols.rs
pub trait Protocol
{
  fn validate_packet(&mut self, packet: &PcapPacket) -> Result<(), String>;
}

pub fn to_pcap(packet: Packet) -> PcapPacket<'static>
{
  return  PcapPacket::new_owned(
    Duration::from_secs_f64(timeval_to_i64(packet.header.ts)),
    packet.header.caplen,
    packet.data.into()
  )
}

pub fn timeval_to_i64(timeval: timeval) -> f64
{
  let μ = 1e-6;
  return timeval.tv_sec as f64 + (μ * timeval.tv_usec as f64);
}

pub fn get_timestamp() -> String
{
  let system_time = SystemTime::now();
  let datetime: DateTime<Utc> = system_time.into();
  return datetime.format("%Y-%m-%d_%T.%.5f").to_string();
}

/// Extracting the ether type as a u16 number by right shifting the values.
/// [source](https://stackoverflow.com/questions/50243866/how-do-i-convert-two-u8-primitives-into-a-u16-primitive#answer-50244328)
pub fn bytes_to_u16(first_byte: u8, second_byte: u8) -> u16
{ return ((first_byte as u16) << 8) | second_byte as u16; }

pub fn create_capture_device(config: &Config) -> Capture<Active>
{
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

  let device = match devices
    .iter()
    .find(|device| device.name.contains(config.interface.as_str()))
  {
    Some(first_device) =>
    {
      println!("[ {} ] device {} found", "OK".green(), first_device.name);
      first_device.to_owned()
    }
    None =>
    {
      println!(
        "[ {} ] device {} not found in device list. Available devices are: {:?}",
        "ERROR".red(),
        config.interface,
        devices.iter().map(|device| device.name.to_owned()).collect::<Vec<String>>(),
      );
      exit(-1);
    }
  };

  let capture_inactive = match Capture::from_device(device)
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

  match capture.filter(config.filter.as_str(), true)
  {
    Ok(_) => println!("[ {} ] filters set and compiled", "OK".green()),
    Err(error) =>
    {
      println!("[ {} ] couldn't set filters b/c: {}", "ERROR".red(), error);
      exit(-1);
    }
  }

  return capture;
}

pub fn get_ether_type(byte: u16) -> String
{
  return match byte
  {
    0x0800 => "IPv4".to_string(),
    0x0804 => "Chaosnet".to_string(),
    0x0806 => "ARP".to_string(),
    0x0842 => "Wake-on-LAN".to_string(),
    0x22ea => "Stream Reservation Protocol".to_string(),
    0x22f0 => "AVTP".to_string(),
    0x22f3 => "IETF TRILL Protocol".to_string(),
    0x6002 => "DEC MOP RC".to_string(),
    0x6003 => "DECnet Phase IV.to_string(), DNA Routing".to_string(),
    0x6004 => "DEC LAT".to_string(),
    0x8035 => "RARP".to_string(),
    0x809b => "ApplteTalk".to_string(),
    0x80d5 => "LLC PDU".to_string(),
    0x80f3 => "AARP".to_string(),
    0x8100 => "VLAN".to_string(),
    0x8102 => "SLPP".to_string(),
    0x8103 => "VLACP".to_string(),
    0x8137 => "IPX".to_string(),
    0x8204 => "QNX Qnet".to_string(),
    0x86dd => "IPv6".to_string(),
    0x8808 => "Ethernet flow control".to_string(),
    0x8809 => "LACP".to_string(),
    0x8819 => "CobraNet".to_string(),
    0x8847 => "MPLS unicast".to_string(),
    0x8848 => "MPLS multicast".to_string(),
    0x8863 => "PPPoE Discovery Stage".to_string(),
    0x8864 => "PPPoE Session Stage".to_string(),
    0x887b => "HomePlug".to_string(),
    0x888e => "EAP over LAN".to_string(),
    0x8892 => "PROFINET".to_string(),
    0x889a => "HyperSCSI".to_string(),
    0x88a2 => "ATA over Ethernet".to_string(),
    0x88a4 => "EtherCAT".to_string(),
    0x88a8 => "Service VLAN tag identifier".to_string(),
    0x88ab => "Ethernet Powerlink".to_string(),
    0x88b8 => "GOOSE".to_string(),
    0x88b9 => "GSE".to_string(),
    0x88ba => "SV".to_string(),
    0x88bf => "MikroTik RoMON".to_string(),
    0x88cc => "LLDP".to_string(),
    0x88cd => "SERCOS III".to_string(),
    0x88e1 => "HomePlug Green PHY".to_string(),
    0x88e3 => "Media Redundancy Protocol".to_string(),
    0x88e5 => "MACsec".to_string(),
    0x88e7 => "PBB".to_string(),
    0x88f7 => "PTP".to_string(),
    0x88f8 => "NC-SI".to_string(),
    0x88fb => "PRP".to_string(),
    0x8902 => "CFM".to_string(),
    0x8906 => "FCoE".to_string(),
    0x8914 => "FCoE initialization protocol".to_string(),
    0x8915 => "RoCE".to_string(),
    0x891d => "TTE".to_string(),
    0x893a => "1905.1 IEEE Protocol".to_string(),
    0x892f => "HSR".to_string(),
    0x9000 => "Ethernet Configuration Testing Protocol".to_string(),
    0x9100 | 0x9200 => "Service VLAN tag identifier (S-Tag) on Q-in-Q tunnel".to_string(),
    0xf1c1 => "Redundancy Tag".to_string(),
    _ => "unknown protocol".to_string()
  };
}
