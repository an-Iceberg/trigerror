use std::path::PathBuf;
use crate::utils::OutFormat;

pub struct Writer
{
  out_format: OutFormat,
  out_dir: PathBuf,
}
