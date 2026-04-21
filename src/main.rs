#![allow(clippy::needless_return)]
#![allow(mixed_script_confusables)]

pub mod cli;
pub mod config;
pub mod constants;
pub mod protocols;
pub mod utils;
pub mod mac;
pub mod writer;

use clap::Parser;
use colored::Colorize;
use pcap_file::pcap::{PcapPacket, PcapWriter};
use std::{collections::VecDeque, fs::File, io::Write, path::{Path, PathBuf}, process::exit, time::Duration};
use crate::{cli::CLI, config::Config, constants::DEFAULT_FILE, mac::MAC, protocols::{Protocol, gptp::{GPTP, message_type::MessageType}}, utils::{OutFormat, create_capture_device, get_timestamp, to_pcap, write_error, write_footer, write_header}};

fn main()
{
  // let vec: Vec<u32> = vec![1,2,3,4,5];
  // let result = vec.iter()
  //   .map(λ!{n => n.pow(2)})
  //   .sum::<u32>();
  // dbg!{result};
  // exit(-1);

  // d = height of the dog
  // p = height of the pole
  // p - d = 200cm
  // p + d = 300cm
  // Matrix A = [1,-1;1,1]
  // Vector b = [200;300]
  // Solve: A*x = b, x = [p;d]
  // Solution: p = 250, d = 50

  // println!("{}", 1_000_000.to_string()
  //   .as_bytes()
  //   .rchunks(3)
  //   .rev()
  //   .map(std::str::from_utf8)
  //   .collect::<Result<Vec<&str>, _>>()
  //   .unwrap()
  //   .join("_")
  // );

  // println!("{}", (Duration::from_micros(5) + Duration::from_micros(350).mul_f64(0.7)).as_micros());

  // println!("{}", MAC::from_bytes((123,123,125,48,12,32)));

  // println!("{message_type}", message_type = MessageType::Sync1Step);
  // println!("{message_type}", message_type = MessageType::Sync2Step);
  // println!("{message_type}", message_type = MessageType::Announce);
  // println!("{message_type}", message_type = MessageType::FollowUp);
  // println!("{message_type}", message_type = MessageType::Signaling);

  // exit(32);

  let cli = CLI::parse();

  // Writes default config file to current working directory.
  if cli.create_default_config
  {
    if Path::new("trigerror.ini").exists()
    {
      eprintln!("[ {} ] file exists already. Not overwriting.", "WARN".yellow());
      exit(0);
    }

    let mut file = match File::create("trigerror.ini")
    {
      Ok(file) => file,
      Err(error) =>
      {
        eprintln!("[ {} ] couldn't create file b/c: {}", "ERROR".red(), error);
        exit(-1);
      },
    };
    match file.write_all(DEFAULT_FILE.as_bytes())
    {
      Ok(_) => (),
      Err(error) => eprintln!("[ {} ] couldn't write to file b/c: {}", "ERROR".red(), error),
    };
    exit(0);
  }

  let mut interfaces = vec![];
  let mut config = Config::new();

  // CONFIGURE PHASE

  println!("[ {} ] configuring …", "INFO".cyan());

  // Check if config file location has been specified.
  if let Some(config_file_path) = cli.config_file_location
  {
    // Read config from said location and determine interfaces.
    if let Some(ifaces) = config.set_from_ini(config_file_path)
    { interfaces = ifaces; }
  }
  else
  {
    // Read config file in cwd (if exists).
    if let Some(ifaces) = config.set_from_ini(PathBuf::from("trigerror.ini"))
    { interfaces = ifaces; }

    // Overwrite options from CLI.
    if let Some(ifaces) = config.set_from_cli(cli)
    { interfaces = ifaces; }
  }

  println!("[ {} ] done configuring", "INFO".cyan());

  // MONITORING PHASE

  // TODO: create a thread for each interface. HELP: https://doc.rust-lang.org/rust-by-example/std_misc/threads.html

  {
    println!("[ {} ] setting up capture device", "INFO".cyan());

    let first_iface = interfaces.first().unwrap();
    config.interface = first_iface.to_owned();
    let mut capture_device = create_capture_device(&config);
    // Allocate .with_capacity().
    let mut buffer: VecDeque<PcapPacket> = VecDeque::with_capacity(config.count_before as usize);
    let mut protocol = GPTP::new();
    // let mut recording = Recording::new(config);

    println!("[ {} ] monitoring …", "INFO".cyan());

    // TODO: this is formulated imperatively just so it works. Use better structures.
    loop
    {
      let packet = to_pcap(capture_device.next_packet().unwrap());

      match protocol.validate_packet(&packet)
      {
        Ok(_) =>
        {
          let current_packet_time = packet.timestamp;

          buffer.push_back(packet);

          // TODO: time is prio #1
          // NOTE: this is a constantly moving time window. It can happen, that the buffer barely has anything in it but it
          // NOTE: doesn't get filled up further b/c the next packet arrived so much later that all packets in the buffer expired.
          // Discard packets that are too old.
          while current_packet_time.abs_diff(buffer.front().unwrap().timestamp).as_millis() > config.time_before as u128
          { buffer.pop_front().unwrap(); }

          // Discard packets that make the buffer too big.
          while buffer.len() as u32 > config.count_before
          { buffer.pop_front().unwrap(); }
        }
        Err(errors) =>
        {
          // WRITING PHASE

          println!("[ {} ] writing to files", "INFO".cyan());

          let interface = config.interface.to_owned();
          let timestamp = get_timestamp();

          // Creating the capture file.
          let mut out_path = config.out_dir.clone();
          out_path.push(format!("trigerror_{interface}_{timestamp}.pcap"));
          let capture_file = File::create(out_path).expect("couldn't create packet capture file");
          let mut capture_writer = PcapWriter::new(capture_file).expect("Error writing to capture file");

          // Creating a file with information about the errors.
          let mut out_path = config.out_dir.clone();
          out_path.push(format!(
            "trigerror_{interface}_{timestamp}.errors.{}",
            match config.out_format
            {
              OutFormat::Text => "txt",
              OutFormat::CSV => "csv",
              OutFormat::JSON => "json",
            }
          ));
          let mut info_file = File::create(out_path).expect("couldn't create errors file");

          // NOTE: This might be very slow.
          // Writing buffer to capture file.
          for packet in buffer.iter()
          { capture_writer.write_packet(packet).expect("Error writing to capture file"); }
          // Writing the packet that triggered the error.
          capture_writer.write_packet(&packet).unwrap();

          // Creating capture control variables.
          let mut packet_number = buffer.len() + 1;
          let mut packet_counter = 0;
          let mut error_time = packet.timestamp;
          let mut δ_time;
          let mut retrigger_counter = 1;
          let mut error_id = 1;

          write_header(&mut info_file, config.out_format);

          write_error(&mut error_id, packet_number, &errors, &mut info_file, config.out_format);
          // error_id += 1;

          // Write network traffic to capture and info file.
          loop
          {
            // FIX: This seems to skip an announce message.
            let packet = to_pcap(capture_device.next_packet().unwrap());
            packet_number += 1;
            capture_writer.write_packet(&packet).unwrap();

            // Another error happened! Record info about it.
            if let Err(errors) = protocol.validate_packet(&packet)
            {
              write_error(&mut error_id, packet_number, &errors, &mut info_file, config.out_format);
              // error_id += 1;

              // Handle retrigger behavior.
              if config.retrigger && retrigger_counter < config.max_retriggers
              {
                println!("[ {} ] retrigger!", "INFO".cyan());
                retrigger_counter += 1;
                packet_counter = 0;
                δ_time = Duration::from_millis(0);
                error_time = packet.timestamp;
              }
            }

            // End of capture conditions.

            δ_time = packet.timestamp.abs_diff(error_time);
            if δ_time.as_millis() > config.time_after as u128
            {
              println!("[ {} ] end of writing b/c time_after exceeded", "INFO".cyan());
              break;
            }

            packet_counter += 1;
            if packet_counter > config.count_after
            {
              println!("[ {} ] end of writing b/c count_after exceeded", "INFO".cyan());
              break;
            }
          }

          buffer.clear();

          // Write the end of the errors file.
          write_footer(&mut info_file, config.out_format);

          // exit(0);
        }
      }
    }
  }
}
