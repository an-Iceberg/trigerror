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

#[derive(Debug)]
pub struct Config
{
  /// The interface(s), from which packets should be read.
  pub interface: String,
  /// These are the protocols that trigger a capture when an error happens in them.
  pub protocols: Vec<String>,
  /// Path where the captured data is stored as a `.pcap` file.
  pub capture_files_path: PathBuf,
  /// Only record these additional protocols.
  /// If empty list, then record no additional protocols.
  /// If None, record everything.
  pub filters: String,
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
      filters: "".to_string(),
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

impl Config
{
  pub fn new() -> Self { return Self::default(); }

  pub fn set_from_ini(&mut self, path: PathBuf)
  {

  }

  pub fn set_from_cli(&mut self, cli: CLI)
  {

  }
}
