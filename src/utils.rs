use clap::ValueEnum;
use chrono::{DateTime, Utc};
use colored::Colorize;
use crate::config::Config;
use libc::timeval;
use pcap::{Active, Capture, Device, Packet};
use pcap_file::pcap::PcapPacket;
use std::{fs::File, io::Write, process::exit, time::{Duration, SystemTime}};

#[derive(ValueEnum, Clone, Copy, Default, Debug)]
pub enum OutFormat { #[default] Text, CSV, JSON }

pub type Octet = u8;

/// Macro for syntactically more pleasing lambda functions/closures.
///
/// Preferably write it like this: `λ!{n => n*2}`.
///
/// ---
///
/// I would have loved to implement syntax that looks like this: `λ!{n -> n*2}`
/// but Rust doesn't allow such syntax :(
#[macro_export]
macro_rules! λ
{
  ( $($variable:ident $(: $type:ty)?),* => $expression:expr ) =>
  {
    |$($variable $(: $type)?),*| $expression
  }
}

pub fn duration_to_string(timestamp: Duration) -> String
{
  let seconds = timestamp.as_secs_f64();
  let datetime = DateTime::from_timestamp(
    seconds.trunc() as i64,
    seconds.fract() as u32
  ).unwrap();
  // FIX: the subsecond part is not being printed.
  return datetime.format("%Y-%m-%d_%T%.f").to_string();
}

/// Returns, whether the bit at `index` is set in the byte `byte`.
pub fn get_bit(byte: u8, index: usize) -> bool
{
  return (byte >> index & 0x0000_0001) == 0x0000_0001;
}

/// Converts a `Packet` struct from a `pcap::Capture` device to a `PcapPacket`
/// which can easily be used for writing packets to a capture file.
pub fn to_pcap(packet: Packet) -> PcapPacket<'static>
{
  return  PcapPacket::new_owned(
    timeval_to_duration(packet.header.ts),
    packet.header.caplen,
    packet.data.into()
  )
}

/// Converts a C `timeval` to a `std::time::Duration`.
///
/// [Source](https://man7.org/linux/man-pages/man3/timeval.3type.html)
pub fn timeval_to_duration(timeval: timeval) -> Duration
{
  let μ = 1e-6;
  return Duration::from_secs_f64(timeval.tv_sec as f64 + (μ * timeval.tv_usec as f64));
}

/// Returns the current timestamp as a formatted `String`.
pub fn get_timestamp() -> String
{
  let system_time = SystemTime::now();
  let datetime: DateTime<Utc> = system_time.into();
  // NOTE: see if the "%f" works.
  return datetime.format("%Y-%m-%d_%T").to_string();
  // return datetime.format("%Y-%m-%d_%T%.f").to_string();
  // This might be how to get milliseconds to show up.
  // return datetime.format("%Y-%m-%d_%T.%.5f").to_string();
}

/// Extracting the ether type as a u16 number by right shifting the values.
///
/// [source](https://stackoverflow.com/questions/50243866/how-do-i-convert-two-u8-primitives-into-a-u16-primitive#answer-50244328)
pub fn bytes_to_u16(first_byte: u8, second_byte: u8) -> u16
{ return ((first_byte as u16) << 8) | second_byte as u16; }

pub fn write_header(file: &mut File, out_format: OutFormat)
{
  match out_format
  {
    OutFormat::CSV => file.write_all("error_id, packet_id, error\n".as_bytes()).unwrap(),
    OutFormat::JSON => file.write_all("[\n".as_bytes()).unwrap(),
    OutFormat::Text => (),
  }
}

pub fn write_error(
  error_id: usize,
  packet_id: usize,
  errors: &Vec<String>,
  file: &mut File,
  out_format: OutFormat,
)
{
  match out_format
  {
    OutFormat::Text => errors.iter().for_each(
      λ!{error =>
        file.write_all(
          format!("error No. {error_id}, packet No. {packet_id}: {error}\n").as_bytes()
        ).unwrap()
      }
    ),
    OutFormat::CSV => errors.iter().for_each(
      λ!{error =>
        file.write_all(
          format!("{error_id}, {packet_id}, \"{error}\"\n").as_bytes()
        ).unwrap()
      }
    ),
    OutFormat::JSON =>
    {
      for error in errors
      {
        file.write_all(format!(
          "  {{\n    \"error_id\": {error_id},\n    \"packet_id\": {packet_id},\n    \"error\": \"{error}\"\n  }},\n"
        ).as_bytes()).unwrap()
      }
    }
  }
}

pub fn write_footer(file: &mut File, out_format: OutFormat)
{
  if matches!(out_format, OutFormat::JSON)
  { file.write_all("]".as_bytes()).unwrap(); }
}

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
      eprintln!("[ {} ] couldn't list devices b/c: {}", "ERROR".red(), error);
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
      eprintln!(
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
      eprintln!("[ {} ] couldn't create capture device b/c: {}", "ERROR".red(), error);
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
      eprintln!("[ {} ] couldn't open capture device b/c: {}", "ERROR".red(), error);
      exit(-1);
    }
  };

  match capture.filter(config.filter.as_str(), true)
  {
    Ok(_) => println!("[ {} ] filters set and compiled", "OK".green()),
    Err(error) =>
    {
      eprintln!("[ {} ] couldn't set filters b/c: {}", "ERROR".red(), error);
      exit(-1);
    }
  }

  return capture;
}
