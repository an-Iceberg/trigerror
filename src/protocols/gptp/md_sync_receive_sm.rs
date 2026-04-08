use std::time::Duration;
use crate::protocols::gptp::message::GPTPMesage;

#[derive(Debug)]
enum State
{
  WaitingForFollowUp,
  WaitingForSync,
  Uninitialized,
}

// Sync and follow up
// TODO: wait for sync messages (messageType == Sync) and then for follow up (messageType == follow up)
// NOTE: log_message_interval
// NOTE: if log_message_interval changes, error and set value of erroneous packet to new now value
// TODO: time margin 30%
// Sync timeout, frame comes periodically, record when packet is missing (datafield last_sync_timer)
// Figure 11-6

pub struct MDSyncReceiveStateMachine
{
  state: State,
  message_interval: Duration,
  last_message_timestamp: Duration,
  margin: f64,
}

impl MDSyncReceiveStateMachine
{
  pub fn new(log_message_interval: Duration) -> Self
  {
    return MDSyncReceiveStateMachine
    {
      state: State::WaitingForSync,
      message_interval: log_message_interval,
      last_message_timestamp: Duration::default(),
      margin: 0.3,
    };
  }

  pub fn change_state(&mut self, timestamp: Duration, message: GPTPMesage) -> Result<(), String>
  {
    let result;

    // dbg!{(&self.state, &message)};

    match (&self.state, message)
    {
      (
        &State::Uninitialized,
        GPTPMesage::Sync1Step { header, .. }
        | GPTPMesage::Sync2Step { header, .. }
        | GPTPMesage::FollowUp { header }
      ) =>
      {
        // TODO: initialize the state machine.
        self.message_interval = header.message_interval();
        result = Ok(());
      },

      (&State::WaitingForSync, GPTPMesage::Sync1Step { header, .. }) =>
      {
        self.state = State::WaitingForFollowUp;

        // Verify that message is within specified time
        if self.last_message_timestamp + self.message_interval.mul_f64(1.0 - self.margin) < timestamp
        {
          result = Err(format!(
            "1 step sync came in {}μs too early.",
            (self.last_message_timestamp + self.message_interval.mul_f64(1.0 - self.margin))
              .abs_diff(timestamp)
              .as_micros(),
          ));
        }
        else if self.last_message_timestamp + self.message_interval.mul_f64(1.0 + self.margin) > timestamp
        {
          result = Err(format!(
            "1 step sync came in {}μs too late.",
            (self.last_message_timestamp + self.message_interval.mul_f64(1.0 + self.margin))
              .abs_diff(timestamp)
              .as_micros(),
          ));
        }
        else { result = Ok(()); }

        self.message_interval = header.message_interval();
      },

      (&State::WaitingForSync, GPTPMesage::Sync2Step { header, .. }) =>
      {
        self.state = State::WaitingForFollowUp;

        // Verify that message is within specified time
        if self.last_message_timestamp + self.message_interval.mul_f64(1.0 - self.margin) < timestamp
        {
          result = Err(format!(
            "2 step sync came in {}μs too early.",
            (self.last_message_timestamp + self.message_interval.mul_f64(1.0 - self.margin))
              .abs_diff(timestamp)
              .as_micros(),
          ));
        }
        else if self.last_message_timestamp + self.message_interval.mul_f64(1.0 + self.margin) > timestamp
        {
          result = Err(format!(
            "2 step sync came in {}μs too late.",
            (self.last_message_timestamp + self.message_interval.mul_f64(1.0 + self.margin))
              .abs_diff(timestamp)
              .as_micros(),
          ));
        }
        else { result = Ok(()); }

        self.message_interval = header.message_interval();
      },

      (&State::WaitingForFollowUp, GPTPMesage::FollowUp { header }) =>
      {
        self.state = State::WaitingForSync;

        // Verify that message is within specified time
        if self.last_message_timestamp + self.message_interval.mul_f64(1.0 - self.margin) < timestamp
        {
          result = Err(format!(
            "follow up came in {}μs too early.",
            (self.last_message_timestamp + self.message_interval.mul_f64(1.0 - self.margin))
              .abs_diff(timestamp)
              .as_micros(),
          ));
        }
        else if self.last_message_timestamp + self.message_interval.mul_f64(1.0 + self.margin) > timestamp
        {
          result = Err(format!(
            "follow up came in {}μs too late.",
            (self.last_message_timestamp + self.message_interval.mul_f64(1.0 + self.margin))
              .abs_diff(timestamp)
              .as_micros(),
          ));
        }
        else { result = Ok(()); }

        self.message_interval = header.message_interval();
      },

      // TODO: better error message 😆.
      _ => result = Err("wtf⁈".to_string())
    };

    self.last_message_timestamp = timestamp;

    return result;
  }
}
