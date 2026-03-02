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
  // The place in the file system, where the `.pcap` file should be written to.
  pub capture_files_location: PathBuf,

  // The interface(s), from which packets should be read.
  pub interfaces: Vec<String>,

  // The file which is used to configure trigerror
  pub config_file_location: Option<PathBuf>,

  pub filters: Option<Vec<String>>,
  pub count_before: Option<u32>,
  pub count_after: Option<u32>,
  pub time_before: Option<f32>,
  pub time_after: Option<f32>,
  pub retrigger: Option<bool>,
}
