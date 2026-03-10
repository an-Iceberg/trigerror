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
  /// The file which is used to configure trigerror. If this option is given then
  /// all other configurations thru the CLI will be ignored.
  #[arg(long, short = 'o')]
  pub config_file_location: Option<PathBuf>,

  // TODO: turn this into just a String and parse it into a Vec later.
  /// The interface(s), from which packets should be read.
  /// This argument is parsed as a String and gets split into a list later on.
  #[arg(long, short = 'i')]
  pub interfaces: Option<String>,

  // TODO: turn this into just a String and parse it into a Vec later.
  /// These are the protocols we want to monitor for errors.
  /// This argument is parsed as a String and gets split into a list later on.
  #[arg(long, short = 'p')]
  pub protocols: Option<String>,

  /// The place in the file system, where the `.pcap` file should be written to.
  /// The default directory is the `cwd`.
  #[arg(long, short = 'a')]
  pub capture_files_path: Option<PathBuf>,

  // TODO: turn this into just a String and parse it into a Vec later.
  /// Only these protocols shall be recorded.
  /// This argument is parsed as a String and gets split into a list later on.
  #[arg(long, short = 'f')]
  pub filters: Option<String>,

  /// How many packets before the error should be recorded.
  #[arg(long)]
  pub count_before: Option<u32>,

  /// How many packets after the error should be recorded.
  #[arg(long)]
  pub count_after: Option<u32>,

  /// How many milliseconds before the error should the recording start.
  #[arg(long)]
  pub time_before: Option<u32>,

  /// How many milliseconds after the error should the recording stop.
  #[arg(long)]
  pub time_after: Option<u32>,

  /// If true and if errors happens after our initial error then the counter and
  /// timer get reset.
  #[arg(long, short = 'r')]
  pub retrigger: Option<bool>,

  /// The maximum amount of errors that should be recorded.
  #[arg(long, short = 'm')]
  pub max_retriggers: Option<u32>,
}
