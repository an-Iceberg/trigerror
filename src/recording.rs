use std::{collections::VecDeque, fs::File, path::PathBuf, process::exit, time::Duration};
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
}, get_timestamp, ring_buffer::RingBuffer, timeval_to_i64};

pub struct Recording
{
  // TODO: one struct has one interface and has one file path.
  // each provided interface creates its own struct.

  pub(crate) config: Config,
  capture: Capture<Active>,
  buffer: Vec<PcapPacket<'static>>,

  /// The interface(s), from which packets should be read.
  pub interfaces: Vec<String>,
  /// These are the protocols that trigger a capture when an error happens in them.
  pub protocols: Vec<String>,
  /// Path where the captured data is stored as a `.pcap` file.
  pub capture_files_path: PathBuf,
  /// Only record these additional protocols.
  /// If empty list, then record no additional protocols.
  /// If None, record everything.
  pub filters: Option<String>,
  /// How many packets before the error should be recorded.
  pub count_before: u32,
  /// How many packets after the error should be recorded.
  pub count_after: u32,
  /// How many milliseconds before the error should the recording start.
  pub time_before: u32,
  /// How many milliseconds after the error should the recording stop.
  pub time_after: u32,
  /// How large in MB the size should be.
  pub file_size: u32,
  /// If true and if errors happens after our initial error then the counter and timer get reset.
  pub retrigger: bool,
  /// The maximum amount of errors that should be recorded.
  pub max_retriggers: u32,
}

// FIX: it might not be a good idea to impl Default for Recording.
impl Default for Recording
{
  fn default() -> Self
  {
    return Self
    {
      config: Config::default(),
      capture: Device::lookup().unwrap().unwrap().open().unwrap(),
      buffer: vec![],
      interfaces: Vec::default(),
      filters: None,
      capture_files_path: PathBuf::from("."),
      protocols: Vec::default(),
      count_before: DEFAULT_COUNT_BEFORE,
      count_after: DEFAULT_COUNT_AFTER,
      time_before: DEFAULT_TIME_BEFORE,
      time_after: DEFAULT_TIME_AFTER,
      file_size: DEFAULT_FILE_SIZE,
      retrigger: DEFAULT_RETRIGGER,
      max_retriggers: DEFAULT_MAX_RETRIGGERS,
    };
  }
}

impl Recording
{
  pub fn new() -> Self { return Self::default(); }

  pub fn setup(&mut self)
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
      .find(|device| device.name.contains(self.config.interface.as_str()))
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
          self.config.interface,
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

    self.capture = match capture_inactive.open()
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

    match self.capture.filter(&self.config.filters, true)
    {
      Ok(_) => println!("[ {} ] filters set and compiled", "OK".green()),
      Err(error) =>
      {
        println!("[ {} ] couldn't set filters b/c: {}", "ERROR".red(), error);
        exit(-1);
      }
    }
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
  pub fn give_packet(&mut self) -> Result<PcapPacket<'static>, Error>
  {
    return match self.capture.next_packet()
    {
      Ok(packet) =>
      {
        let packet = PcapPacket::new_owned(
          Duration::from_secs_f64(timeval_to_i64(packet.header.ts)),
          packet.header.caplen,
          packet.data.into()
        );
        // NOTE: this might be inefficient.
        // self.buffer.push(packet.clone());
        Ok(packet)
      }
      Err(error) => Err(error),
    };
  }

  pub fn push_packet(mut self, packet: PcapPacket<'static>)
  { self.buffer.push(packet); }

  pub fn trigger(&mut self, timestamp: String)
  {
    let interface = self.config.interface.to_string().to_string().to_string();
    let outfile = File::create(format!("trigerror_{interface}_{timestamp}.pcap")).expect("couldn't create file");
    let mut pcap_writer = PcapWriter::new(outfile).expect("Error writing file");
    self.buffer.iter().for_each(|packet| { pcap_writer.write_packet(packet).unwrap(); } );
    self.buffer.clear();
  }

  pub fn configure_from_ini(path: PathBuf) -> Recording
  {
    let mut trigerror = Recording::new();

    let config = match ini!(safe path.to_str().unwrap())
    {
      Ok(config) =>
      {
        println!("[ {} ] found config file @ {}", "OK".green(), path.to_str().unwrap());
        config.to_owned()
      }
      Err(error) =>
      {
        println!("[ {} ] didn't find config file b/c: {}", "ERROR".red(), error);
        println!("[ {} ] falling back to default configurations", "INFO".cyan());
        return trigerror;
      }
    };

    let default = match config.get("default")
    {
      Some(default) =>
      {
        // BUG: why does this not print the OK in green⁈
        println!("[ {} ] found config(s)", "OK".green());
        default.to_owned()
      }
      None =>
      {
        println!("[ {} ] no configs in file", "ERROR".red());
        println!("[ {} ] falling back to default configurations", "INFO".cyan());
        return trigerror;
      }
    };

    // NOTE: if you're bored you can add log messages to all these.

    if let Some(Some(interfaces)) = default.get("interfaces")
    {
      trigerror.interfaces = interfaces
        .split(",")
        .map(|interface| interface.trim().to_string())
        .collect();
    }

    if let Some(Some(protocols)) = default.get("protocols")
    {
      trigerror.protocols = protocols
        .split(",")
        .map(|protocol| protocol.trim().to_string())
        .collect();
    }

    if let Some(Some(filters)) = default.get("filters")
    {
      trigerror.filters = Some(filters.to_owned());
    }

    if let Some(count_before) = default.get("count_before")
      .and_then(|count_before| count_before.as_ref())
      .and_then(|count_before| count_before.parse::<u32>().ok())
    { trigerror.count_before = count_before; }

    if let Some(count_after) = default.get("count_after")
      .and_then(|count_after| count_after.as_ref())
      .and_then(|count_after| count_after.parse::<u32>().ok())
    { trigerror.count_after = count_after; }

    if let Some(time_before) = default.get("time_before")
      .and_then(|time_before| time_before.as_ref())
      .and_then(|time_before| time_before.parse::<u32>().ok())
    { trigerror.time_before = time_before; }

    if let Some(time_after) = default.get("time_after")
      .and_then(|time_after| time_after.as_ref())
      .and_then(|time_after| time_after.parse::<u32>().ok())
    { trigerror.time_after = time_after; }

    if let Some(retrigger) = default.get("retrigger")
      .and_then(|retrigger| retrigger.as_ref())
      .and_then(|retrigger| retrigger.parse::<bool>().ok())
    { trigerror.retrigger = retrigger; }

    if let Some(max_retriggers) = default.get("max_retriggers")
      .and_then(|max_retriggers| max_retriggers.as_ref())
      .and_then(|max_retriggers| max_retriggers.parse::<u32>().ok())
    { trigerror.max_retriggers = max_retriggers; }

    return trigerror;
  }

  pub fn configure_from_cli(&mut self, cli: CLI)
  {
    if let Some(config_file) = cli.config_file_location
    {
      println!("[ {} ] config file location given; configuring from said file", "INFO".cyan());
      *self = Recording::configure_from_ini(config_file);
      return;
    }

    // Configuring interfaces thru CLI
    if let Some(interfaces) = cli.interfaces
    {
      self.interfaces = interfaces
        .split(",")
        .map(|interface| interface.trim().to_string())
        .collect();
    }

    // Configuring protocols thru CLI
    if let Some(protocols) = cli.protocols
    {
      self.protocols = protocols
        .split(",")
        .map(|protocol| protocol.trim().to_string())
        .collect();
    }

    if let Some(capture_files_path) = cli.capture_files_path
    { self.capture_files_path = capture_files_path; }

    if let Some(filters) = cli.filters
    {
      // This way one can reset the filters to `None` thru the CLI.
      if filters.is_empty() { self.filters = None; }
      else { self.filters = Some(filters.to_owned()); }
    }

    if let Some(count_before) = cli.count_before
    { self.count_before = count_before; }

    if let Some(count_after) = cli.count_after
    { self.count_after = count_after; }

    if let Some(time_before) = cli.time_before
    { self.time_before = time_before; }

    if let Some(time_after) = cli.time_after
    { self.time_after = time_after; }

    if let Some(retrigger) = cli.retrigger
    { self.retrigger = retrigger; }

    if let Some(max_retriggers) = cli.max_retriggers
    { self.max_retriggers = max_retriggers; }
  }
}
