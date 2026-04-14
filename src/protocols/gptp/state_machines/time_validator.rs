use std::time::Duration;
use crate::{constants::MARGIN, protocols::gptp::message_type::MessageType};

#[derive(Default)]
pub struct TimeValidator
{
  message_interval: Duration,
  last_message_timestamp: Duration,
}

impl TimeValidator
{
  pub fn validate(
    &mut self,
    current_message_timestamp: Duration,
    new_message_interval: Duration,
    // message_type: MessageType,
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

    let last_message_timestamp = self.last_message_timestamp;
    let message_interval = self.message_interval;

    // Calculate relevant values.
    // This adds 70% of the message interval.
    let lower_bound = last_message_timestamp + message_interval.mul_f64(1. - MARGIN);
    // This adds 130% of the message interval.
    let upper_bound = last_message_timestamp + message_interval.mul_f64(1. + MARGIN);

    // Update state.
    self.last_message_timestamp = current_message_timestamp;
    self.message_interval = new_message_interval;

    // Return result.
    if current_message_timestamp < lower_bound
    {
      let diff = current_message_timestamp.abs_diff(lower_bound).as_micros() as f64 / 1_000.;
      return Err(format!("Message came in {diff:.3}ms too early."));
    }
    else if upper_bound < current_message_timestamp
    {
      let diff = upper_bound.abs_diff(current_message_timestamp).as_micros() as f64 / 1_000.;
      return Err(format!("Message came in {diff:.3}ms too late."));
    }
    return Ok(());
  }
}
