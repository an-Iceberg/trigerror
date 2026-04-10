use std::{fmt::Display, time::Duration};

use crate::{utils::duration_to_string, protocols::gptp::message_type::MessageType};

#[derive(Debug, Default, Clone, Copy)]
enum State
{
  #[default]
  WaitingForAnnounce,
}

impl Display for State
{
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    return formatter.write_str(match self
    {
      State::WaitingForAnnounce => "WaitingForAnnounce"
    });
  }
}

pub struct AnnounceStateMachine
{
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

  pub fn validate_state(&mut self, message_type: MessageType) -> Result<(), String>
  {
    use State::{WaitingForAnnounce};
    use MessageType::Announce;

    return match (self.state, message_type)
    {
      // Expected state changes.
      (WaitingForAnnounce, Announce) => { Ok(()) }

      // Unexpected state changes.

      // Catchall
      (state, message_type) => Err(format!(
        "Unknown state and message combination: state: {state}, message type: {message_type:?}"
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
       ┊                                                  ┊
    */

    // Calculate relevant values.
    let lower_bound = self.last_message_timestamp + self.message_interval.mul_f64(1. - self.margin);
    let upper_bound = self.last_message_timestamp + self.message_interval.mul_f64(1. + self.margin);

    // Update state.
    self.last_message_timestamp = current_message_timestamp;
    self.message_interval = new_message_interval;

    // Return result.
    if current_message_timestamp < lower_bound
    {
      let diff = current_message_timestamp.abs_diff(lower_bound).as_micros() as f64 / 1_000.;

      return Err(format!(
        "{message_type:?} came in {:.3}ms too early. Lower bound: {}, actual: {}",
        diff,
        duration_to_string(lower_bound),
        duration_to_string(current_message_timestamp)
      ));
    }
    else if upper_bound < current_message_timestamp
    {
      let diff = upper_bound.abs_diff(current_message_timestamp).as_micros() as f64 / 1_000.;

      return Err(format!(
        "{message_type:?} came in {:.3}ms too late. Upper bound: {}, actual: {}",
        diff,
        duration_to_string(upper_bound),
        duration_to_string(current_message_timestamp)
      ));
    }

    return Ok(());
  }
}
