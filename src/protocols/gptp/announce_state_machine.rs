use std::{fmt::Display, time::Duration};

use crate::{mac::MAC, protocols::gptp::message_type::MessageType, utils::duration_to_string};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum State
{
  #[default]
  Uninitialized,
  WaitingForAnnounce,
}

impl Display for State
{
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    return formatter.write_str(match self
    {
      State::Uninitialized => "Uninitialized",
      State::WaitingForAnnounce => "WaitingForAnnounce",
    });
  }
}

pub struct AnnounceStateMachine
{
  source_mac: MAC,
  state: State,
  message_interval: Duration,
  last_message_timestamp: Duration,
  margin: f64,
}

impl Default for AnnounceStateMachine
{
  fn default() -> Self
  {
    return AnnounceStateMachine
    {
      source_mac: MAC::default(),
      state: State::default(),
      message_interval: Duration::default(),
      last_message_timestamp: Duration::default(),
      margin: 0.3,
    };
  }
}

impl AnnounceStateMachine
{
  pub fn new() -> Self { return Default::default(); }

  pub fn is_uninitialized(&self) -> bool { return self.state == State::Uninitialized; }

  pub fn validate_state(&mut self, message_type: MessageType) -> Result<(), String>
  {
    use State::{WaitingForAnnounce, Uninitialized};
    use MessageType::Announce;

    return match (self.state, message_type)
    {
      // Expected state changes.
      (Uninitialized, Announce) => { Ok(()) }
      (WaitingForAnnounce, Announce) => { Ok(()) }

      // Unexpected state changes.

      // Catchall
      (state, message_type) => Err(format!(
        "Unknown state and message combination from AnnounceSM: state: {state}, message type: {message_type:?}"
      ))
    };
  }

  pub fn validate_timing(
    &mut self,
    current_message_timestamp: Duration,
    new_message_interval: Duration,
    message_type: MessageType,
  ) -> Result<(), String>
  {
    /*
     last_message_timestamp                    current_message_timestamp
       │                                                  │
     ──┼──────────────────────────────────────────────────┼──────────────────> time
       │                                                  │
       ┊                                                  ┊
       ┊         message_interval                         ┊
       ├───────────────────────────────┤                  ┊
       ┊                             margin               ┊
       ┊                         ├───────────┤            ┊
       ┊                  lower_bound    upper_bound      ┊
    */

    // Calculate relevant values.
    // This adds 70% of the message interval.
    let lower_bound = self.last_message_timestamp + self.message_interval.mul_f64(1. - self.margin);
    // This adds 130% of the message interval.
    let upper_bound = self.last_message_timestamp + self.message_interval.mul_f64(1. + self.margin);

    // Update state.
    self.last_message_timestamp = current_message_timestamp;
    self.message_interval = new_message_interval;

    // Return result.
    if current_message_timestamp < lower_bound
    {
      let diff = current_message_timestamp.abs_diff(lower_bound).as_micros() as f64 / 1_000.;

      return Err(format!("{message_type:?} came in {diff:.3}ms too early."));
    }
    else if upper_bound < current_message_timestamp
    {
      let diff = upper_bound.abs_diff(current_message_timestamp).as_micros() as f64 / 1_000.;

      return Err(format!("{message_type:?} came in {diff:.3}ms too late."));
    }

    return Ok(());
  }

  pub fn validate_mac(&mut self, new_source_mac: MAC) -> Result<(), String>
  {
    if new_source_mac != self.source_mac
    {
      let old_source_mac = self.source_mac;
      self.source_mac = new_source_mac;

      return Err(format!(
        "source MAC address has changed. Was: {}, now is: {}",
        old_source_mac,
        new_source_mac,
      ));
    }

    return Ok(());
  }
}
