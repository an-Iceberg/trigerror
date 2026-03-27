use std::{collections::{VecDeque, vec_deque}, fs::File, io::Write, path::PathBuf, process::exit, time::Duration};
use pcap::{Activated, Active, Capture, Device, Error, Inactive, Packet};
use colored::Colorize;
use ini::ini;
use pcap_file::pcap::{PcapPacket, PcapWriter};
use crate::{cli::CLI, config::Config, constants::{
  DEFAULT_COUNT_AFTER,
  DEFAULT_COUNT_BEFORE,
  DEFAULT_FILE_SIZE,
  DEFAULT_MAX_RETRIGGERS,
  DEFAULT_RETRIGGER,
  DEFAULT_TIME_AFTER,
  DEFAULT_TIME_BEFORE,
}, get_timestamp, to_pcap, ring_buffer::RingBuffer, timeval_to_i64, writer::Writer};

pub enum Source { Internal, External }

enum State { Monitoring, Writing(Writer) }

/// Documentation
pub struct Recording
{
  // TODO: one struct has one interface and has one out dir.
  // each provided interface creates its own recording.

  /// The configuration parameters.
  pub(crate) config: Config,

  /// The capture device from which packets are read
  capture: Capture<Active>,

  /// This acts as a ring buffer that stores packets temporarily until they are needed.
  buffer: VecDeque<PcapPacket<'static>>,

  state: State,

  current_packet: PcapPacket<'static>,
}

// // FIX: it might not be a good idea to impl Default for Recording.
// impl Default for Recording
// {
//   fn default() -> Self
//   {
//     return Self
//     {
//       config: Config::default(),
//       capture: Device::lookup().unwrap().unwrap().open().unwrap(),
//       buffer: vec![],
//     };
//   }
// }

impl Recording
{
  // TODO: add capacity to buffer.
  pub fn new(config: Config) -> Self
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

    return Recording
    {
      config,
      capture,
      buffer: VecDeque::new(),
      state: State::Monitoring,
      current_packet: PcapPacket
      {
        timestamp: Duration::from_secs(0),
        orig_len: 0,
        data: vec![].into()
      }
    };
  }

  // pub fn start(&mut self)
  // {
  //   while let Ok(packet) = self.capture.next_packet()
  //   {
  //     let packet = PcapPacket::new_owned(
  //       Duration::from_secs_f64(timeval_to_i64(packet.header.ts)),
  //       packet.header.caplen,
  //       packet.data.into()
  //     );
  //   }
  // }

  // FIX: static lifetime may cause the heap to become massively bloated b/c it makes an object live for the
  // FIX: entire duration of the program's lifetime. Either clear the buffers surely or use a different lifetime
  // FIX: specifier.
  pub fn get_packet_ref(&mut self) -> Result<&PcapPacket<'static>, Error>
  {
    match self.capture.next_packet()
    {
      Ok(packet) => { self.current_packet = to_pcap(packet); }
      Err(error) => return Err(error),
    };

    match &mut self.state
    {
      State::Monitoring =>
      {
        // FIX: this is very slow b/c .clone()
        self.buffer.push_back(self.current_packet.clone());
        return Ok(self.buffer.back().unwrap());
      }
      State::Writing(writer) =>
      {
        writer.write_packet(&self.current_packet);
        return Ok(&self.current_packet);
      }
    }
  }

  pub fn trigger(&mut self, reason: String, source: Source)
  {
    // let outfile = File::create(format!("trigerror_{interface}_{timestamp}.pcap")).expect("couldn't create file");
    // let mut pcap_writer = PcapWriter::new(outfile).expect("Error writing file");
    // self.buffer.iter().for_each(|packet| { pcap_writer.write_packet(packet).unwrap(); } );
    // self.buffer.clear();


    // match &mut self.state
    // {
    //   State::Monitoring =>
    //   {
    //     let capture_file = File::create(format!("trigerror_{interface}_{timestamp}.pcap")).expect("couldn't create capture file");

    //     self.state = State::Writing
    //     {
    //       capture_writer: PcapWriter::new(capture_file).unwrap(),
    //       reasons_file: File::create(format!("trigerror_info_{interface}_{timestamp}.txt")).expect("couldn't create reasons file")
    //     };
    //   }
    //   State::Writing { capture_writer, mut reasons_file } =>
    //   {
    //     reasons_file.write(b"hello world");
    //     todo!()
    //   }
    // }

    let interface = self.config.interface.to_owned();
    let timestamp = get_timestamp();

    match &self.state
    {
      State::Monitoring => todo!(),
      State::Writing(writer) => todo!(),
    }
  }
}
