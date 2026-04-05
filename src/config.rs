use std::path::PathBuf;
use crate::{cli::CLI, constants::{
  DEFAULT_COUNT_AFTER,
  DEFAULT_COUNT_BEFORE,
  DEFAULT_FILE_SIZE,
  DEFAULT_MAX_RETRIGGERS,
  DEFAULT_RETRIGGER,
  DEFAULT_TIME_AFTER,
  DEFAULT_TIME_BEFORE,
}};
use colored::Colorize;
use ini::ini;

/// The configuration parameters for a recording.
#[derive(Debug, Clone)]
pub struct Config
{
  /// The interface(s), from which packets should be read.
  pub interface: String,
  /// Path where the captured data is stored as a `.pcap` file.
  pub out_dir: PathBuf,
  /// The [BPF](https://biot.com/capstats/bpf.html) applied to the capture device.
  pub filter: String,
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

impl Default for Config
{
  fn default() -> Self
  {
    return Self
    {
      interface: "eth0".to_string(),
      filter: "".to_string(),
      out_dir: PathBuf::from("."),
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

impl Config
{
  pub fn new() -> Self { return Self::default(); }

  pub fn set_from_ini(&mut self, path: PathBuf) -> Option<Vec<String>>
  {
    let config = match ini!(safe path.to_str().unwrap())
    {
      Ok(config) =>
      {
        println!("[ {} ] found config file @ {}", "OK".green(), path.to_str().unwrap());
        config.to_owned()
      }
      Err(error) =>
      {
        eprintln!("[ {} ] didn't find config file b/c: {}", "ERROR".red(), error);
        eprintln!("[ {} ] falling back to default configurations", "INFO".cyan());
        return None;
      }
    };

    let default = match config.get("default")
    {
      Some(default) =>
      {
        println!("[ {} ] found config(s)", "OK".green());
        default.to_owned()
      }
      None =>
      {
        eprintln!("[ {} ] no configs in file", "ERROR".red());
        println!("[ {} ] falling back to default configurations", "INFO".cyan());
        return None;
      }
    };

    // NOTE: if you're bored you can add log messages to all these.

    let mut interfaces = vec![];

    if let Some(Some(ifaces)) = default.get("interfaces")
    {
      interfaces = ifaces
        .split(",")
        .map(|interface| interface.trim().to_string())
        .collect();
    }

    if let Some(Some(filter)) = default.get("filter")
    { self.filter = filter.to_owned(); }

    if let Some(count_before) = default.get("count_before")
      .and_then(|count_before| count_before.as_ref())
      .and_then(|count_before| count_before.replace("_", "").parse::<u32>().ok())
    { self.count_before = count_before; }

    if let Some(count_after) = default.get("count_after")
      .and_then(|count_after| count_after.as_ref())
      .and_then(|count_after| count_after.replace("_", "").parse::<u32>().ok())
    { self.count_after = count_after; }

    if let Some(time_before) = default.get("time_before")
      .and_then(|time_before| time_before.as_ref())
      .and_then(|time_before| time_before.replace("_", "").parse::<u32>().ok())
    { self.time_before = time_before; }

    if let Some(time_after) = default.get("time_after")
      .and_then(|time_after| time_after.as_ref())
      .and_then(|time_after| time_after.replace("_", "").parse::<u32>().ok())
    { self.time_after = time_after; }

    if let Some(retrigger) = default.get("retrigger")
      .and_then(|retrigger| retrigger.as_ref())
      .and_then(|retrigger| retrigger.parse::<bool>().ok())
    { self.retrigger = retrigger; }

    if let Some(max_retriggers) = default.get("max_retriggers")
      .and_then(|max_retriggers| max_retriggers.as_ref())
      .and_then(|max_retriggers| max_retriggers.replace("_", "").parse::<u32>().ok())
    { self.max_retriggers = max_retriggers; }

    if !interfaces.is_empty() { return Some(interfaces); }
    else { return None; }
  }

  pub fn set_from_cli(&mut self, cli: CLI) -> Option<Vec<String>>
  {
    let mut interfaces = vec![];

    // Configuring interfaces thru CLI
    if let Some(ifaces) = cli.interfaces
    {
      interfaces = ifaces
        .split(",")
        .map(|interface| interface.trim().to_string())
        .collect();
    }

    if let Some(out_dir) = cli.out_dir
    { self.out_dir = out_dir; }

    if let Some(filter) = cli.filter
    { self.filter = filter; }

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

    if !interfaces.is_empty() { return Some(interfaces); }
    else { return None; }
  }
}
