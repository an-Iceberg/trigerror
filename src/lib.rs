#![allow(clippy::needless_return)]
#![allow(mixed_script_confusables)]

use crate::trigerror::Trigerror;
use ini::ini;

pub mod gptp;
pub mod cli;
pub mod trigerror;
pub mod constants;

pub fn fun()
{
  println!("hello world");
}

pub fn extract_config() -> Trigerror
{
  let mut trigerror = Trigerror::new();

  let config = ini!("trigerror.ini");

  let default = config["default"].clone();

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
  {
    trigerror.set_count_before(count_before);
  }

  if let Some(count_after) = config
    .get("default")
    .and_then(|default| default.get("count_after"))
    .and_then(|count_after| count_after.as_ref())
    .and_then(|count_after| count_after.parse::<u32>().ok())
  {
    trigerror.set_count_after(count_after);
  }

  if let Some(time_before) = config
    .get("default")
    .and_then(|default| default.get("time_before"))
    .and_then(|time_before| time_before.as_ref())
    .and_then(|time_before| time_before.parse::<f32>().ok())
  {
    trigerror.set_time_before(time_before);
  }

  if let Some(time_after) = config
    .get("default")
    .and_then(|default| default.get("time_after"))
    .and_then(|time_after| time_after.as_ref())
    .and_then(|time_after| time_after.parse::<f32>().ok())
  {
    trigerror.set_time_after(time_after);
  }

  if let Some(retrigger) = config
    .get("default")
    .and_then(|default| default.get("retrigger"))
    .and_then(|retrigger| retrigger.as_ref())
    .and_then(|retrigger| retrigger.parse::<bool>().ok())
  {
    trigerror.set_retrigger(retrigger);
  }

  if let Some(max_retriggers) = config
    .get("default")
    .and_then(|default| default.get("max_retriggers"))
    .and_then(|max_retriggers| max_retriggers.as_ref())
    .and_then(|max_retriggers| max_retriggers.parse::<u32>().ok())
  {
    trigerror.set_max_retriggers(max_retriggers);
  }

  return trigerror;
}
