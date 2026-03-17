use std::path::PathBuf;
use pcap::{Activated, Active, Capture, Packet};
use ratatui::style::Stylize;
use ini::ini;
use crate::{cli::CLI, constants::{
  DEFAULT_COUNT_AFTER, DEFAULT_COUNT_BEFORE, DEFAULT_FILE_SIZE, DEFAULT_MAX_RETRIGGERS, DEFAULT_RETRIGGER, DEFAULT_TIME_AFTER, DEFAULT_TIME_BEFORE
}, ring_buffer::RingBuffer};

#[derive(Debug)]
pub struct Trigerror
{
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

impl Default for Trigerror
{
  fn default() -> Self
  {
    return Self
    {
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

impl Trigerror
{
  pub fn new() -> Self { return Self::default(); }

  pub fn configure_from_ini(path: PathBuf) -> Trigerror
  {
    let mut trigerror = Trigerror::new();

    let config = match ini!(safe path.to_str().unwrap())
    {
      Ok(config) =>
      {
        // BUG: why does this not print the OK in green⁈
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
      *self = Trigerror::configure_from_ini(config_file);
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
