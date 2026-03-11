#![allow(clippy::needless_return)]
#![allow(mixed_script_confusables)]

use crate::{cli::CLI, trigerror::Trigerror};
use ini::ini;

pub mod gptp;
pub mod cli;
pub mod trigerror;
pub mod constants;
pub mod owned_packet;

// TODO: this needs to take a file path. That way we can re-use this for the custom config path.
pub fn extract_config_from_ini() -> Trigerror
{
  let mut trigerror = Trigerror::new();

  let config = ini!("trigerror.ini");

  if let Some(Some(interfaces)) = config
    .get("default")
    .and_then(|default| default.get("interfaces"))
  {
    trigerror.set_interfaces(interfaces
      .split(",")
      .map(|interface| interface.trim().to_string())
      .collect()
    )
  }

  if let Some(Some(protocols)) = config
    .get("default")
    .and_then(|default| default.get("protocols"))
  {
    trigerror.set_protocols(
      protocols
        .split(",")
        .map(|protocol| protocol.trim().to_string())
        .collect()
    );
  }

  if let Some(Some(filters)) = config
    .get("default")
    .and_then(|default| default.get("filters"))
  {
    trigerror.set_filters(
      Some(filters
        .split(",")
        .map(|protocol| protocol.trim().to_string())
        .collect()
      )
    );
  }

  if let Some(count_before) = config
    .get("default")
    .and_then(|default| default.get("count_before"))
    .and_then(|count_before| count_before.as_ref())
    .and_then(|count_before| count_before.parse::<u32>().ok())
  { trigerror.set_count_before(count_before); }

  if let Some(count_after) = config
    .get("default")
    .and_then(|default| default.get("count_after"))
    .and_then(|count_after| count_after.as_ref())
    .and_then(|count_after| count_after.parse::<u32>().ok())
  { trigerror.set_count_after(count_after); }

  if let Some(time_before) = config
    .get("default")
    .and_then(|default| default.get("time_before"))
    .and_then(|time_before| time_before.as_ref())
    .and_then(|time_before| time_before.parse::<u32>().ok())
  { trigerror.set_time_before(time_before); }

  if let Some(time_after) = config
    .get("default")
    .and_then(|default| default.get("time_after"))
    .and_then(|time_after| time_after.as_ref())
    .and_then(|time_after| time_after.parse::<u32>().ok())
  { trigerror.set_time_after(time_after); }

  if let Some(retrigger) = config
    .get("default")
    .and_then(|default| default.get("retrigger"))
    .and_then(|retrigger| retrigger.as_ref())
    .and_then(|retrigger| retrigger.parse::<bool>().ok())
  { trigerror.set_retrigger(retrigger); }

  if let Some(max_retriggers) = config
    .get("default")
    .and_then(|default| default.get("max_retriggers"))
    .and_then(|max_retriggers| max_retriggers.as_ref())
    .and_then(|max_retriggers| max_retriggers.parse::<u32>().ok())
  { trigerror.set_max_retriggers(max_retriggers); }

  return trigerror;
}

// NOTE: maybe move this into the `trigerror` struct?
/// A pure function :D
pub fn configure_trigerror_from_cli(cli: CLI, mut trigerror: Trigerror) -> Trigerror
{
  // If config file location was given manually
  if let Some(config_file) = cli.config_file_location
  {
    // Check that file exists.
    // If yes, parse it and configure trigerror.
    todo!("Implement configuring from file in other location");
  }
  else
  {
    // Configuring interfaces thru CLI
    if let Some(interfaces) = cli.interfaces
    {
      trigerror.set_interfaces(interfaces
        .split(",")
        .map(|interface| interface.trim().to_string())
        .collect()
      );
    }

    // Configuring protocols thru CLI
    if let Some(protocols) = cli.protocols
    {
      trigerror.set_protocols(protocols
        .split(",")
        .map(|protocol| protocol.trim().to_string())
        .collect()
      );
    }

    if let Some(capture_files_path) = cli.capture_files_path
    { trigerror.set_capture_files_path(capture_files_path); }

    if let Some(filters) = cli.filters
    {
      // This way one can reset the filters to `None` thru the CLI.
      if filters.is_empty() { trigerror.set_filters(None); }
      else
      {
        trigerror.set_filters(Some(filters
          .split(",")
          .map(|filter| filter.trim().to_string())
          .collect()
        ));
      }
    }

    if let Some(count_before) = cli.count_before
    { trigerror.set_count_before(count_before); }

    if let Some(count_after) = cli.count_after
    { trigerror.set_count_after(count_after); }

    if let Some(time_before) = cli.time_before
    { trigerror.set_time_before(time_before); }

    if let Some(time_after) = cli.time_after
    { trigerror.set_time_after(time_after); }

    if let Some(retrigger) = cli.retrigger
    { trigerror.set_retrigger(retrigger); }

    if let Some(max_retriggers) = cli.max_retriggers
    { trigerror.set_max_retriggers(max_retriggers); }
  }

  return trigerror;
}
