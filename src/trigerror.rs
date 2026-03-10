use std::path::PathBuf;

use crate::constants::{
  DEFAULT_COUNT_AFTER,
  DEFAULT_COUNT_BEFORE,
  DEFAULT_MAX_RETRIGGERS,
  DEFAULT_RETRIGGER,
  DEFAULT_TIME_AFTER,
  DEFAULT_TIME_BEFORE
};

#[derive(Debug, Clone)]
pub struct Trigerror
{
  // TODO: files path (where to store the captured data).

  /// The interface(s), from which packets should be read.
  interfaces: Vec<String>,
  /// These are the protocols we want to listen for.
  protocols: Vec<String>,

  /// Path where the captured data is stored as a `.pcap` file.
  capture_files_path: PathBuf,

  /// Only record these additional protocols (if None then record everything).
  filters: Option<Vec<String>>,
  /// How many packets before the error should be recorded.
  count_before: u32,
  /// How many packets after the error should be recorded.
  count_after: u32,
  /// How many milliseconds before the error should the recording start.
  time_before: u32,
  /// How many milliseconds after the error should the recording stop.
  time_after: u32,
  /// If true and if errors happens after our initial error then the counter and
  /// timer get reset.
  retrigger: bool,
  /// The maximum amount of errors that should be recorded.
  max_retriggers: u32,
}

impl Trigerror
{
  pub fn new() -> Self { return Self::default(); }

  pub fn set_interfaces(&mut self, interfaces: Vec<String>)
  { self.interfaces = interfaces; }

  pub fn set_filters(&mut self, filters: Option<Vec<String>>)
  { self.filters = filters; }

  pub fn set_capture_files_path(&mut self, path: PathBuf)
  { self.capture_files_path = path; }

  pub fn set_protocols(&mut self, protocols: Vec<String>)
  { self.protocols = protocols; }

  pub fn set_count_before(&mut self, count_before: u32)
  { self.count_before = count_before; }

  pub fn set_count_after(&mut self, count_after: u32)
  { self.count_after = count_after; }

  pub fn set_time_before(&mut self, time_before: u32)
  { self.time_before = time_before; }

  pub fn set_time_after(&mut self, time_after: u32)
  { self.time_after = time_after; }

  pub fn set_retrigger(&mut self, retrigger: bool)
  { self.retrigger = retrigger; }

  pub fn set_max_retriggers(&mut self, max_retriggers: u32)
  { self.max_retriggers = max_retriggers; }
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
      retrigger: DEFAULT_RETRIGGER,
      max_retriggers: DEFAULT_MAX_RETRIGGERS,
    };
  }
}
