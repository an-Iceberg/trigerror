use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(
  name = "trigerror",
  version,
  about,
  long_about = None
)]
pub struct CLI
{
  // TODO: turn this into just a String and parse it into a Vec later.
  /// The interface(s), from which packets should be read.
  /// This argument is parsed as a String and gets split into a list later on.
  pub interfaces: String,

  // TODO: turn this into just a String and parse it into a Vec later.
  /// These are the protocols we want to monitor for errors.
  /// This argument is parsed as a String and gets split into a list later on.
  pub protocols: String,

  /// The file which is used to configure trigerror.
  pub config_file_location: Option<PathBuf>,
  /// The place in the file system, where the `.pcap` file should be written to.
  /// The default directory is the `cwd`.
  pub capture_files_location: Option<PathBuf>,
  // TODO: turn this into just a String and parse it into a Vec later.
  /// Only these protocols shall be recorded.
  /// This argument is parsed as a String and gets split into a list later on.
  pub filters: Option<String>,
  /// How many packets before the error should be recorded.
  pub count_before: Option<u32>,
  /// How many packets after the error should be recorded.
  pub count_after: Option<u32>,
  /// How many milliseconds before the error should the recording start.
  pub time_before: Option<f32>,
  /// How many milliseconds after the error should the recording stop.
  pub time_after: Option<f32>,
  /// If true and if errors happens after our initial error then the counter and
  /// timer get reset.
  pub retrigger: Option<bool>,
  /// The maximum amount of errors that should be recorded.
  pub max_retriggers: Option<u32>,
}
