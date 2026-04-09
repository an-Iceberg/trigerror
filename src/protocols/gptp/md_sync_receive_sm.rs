use std::time::Duration;
use crate::{TimeResult, is_on_time, protocols::gptp::message::GPTPMesage};

#[derive(Debug)]
enum State
{
  WaitingForFollowUp,
  WaitingForSync,
  Uninitialized,
}

impl Default for State
{
  fn default() -> Self { return State::Uninitialized; }
}

// Sync and follow up
// TODO: wait for sync messages (messageType == Sync) and then for follow up (messageType == follow up)
// NOTE: log_message_interval
// NOTE: if log_message_interval changes, error and set value of erroneous packet to new now value
// TODO: time margin 30%
// Sync timeout, frame comes periodically, record when packet is missing (datafield last_sync_timer)
// Figure 11-6
// NOTE: if first is followup, just ignore.
// NOTE: Uninit -> Sync received. all follow ups are ignored and state machine is not advanced.

#[derive(Default)]
pub struct MDSyncReceiveStateMachine
{
  state: State,
  message_interval: Duration,
  last_message_timestamp: Duration,
  margin: f64,
}

impl MDSyncReceiveStateMachine
{
  pub fn new() -> Self { return Default::default(); }

  pub fn change_state(&mut self, timestamp: Duration, message: GPTPMesage) -> Result<(), String>
  {
    let result;

    // TODO: better structure!

    match (&self.state, message)
    {
      (
        &State::Uninitialized,
        GPTPMesage::Sync1Step { header, .. }
        | GPTPMesage::Sync2Step { header, .. }
      ) =>
      {
        // TODO: initialize the state machine.
        self.last_message_timestamp = timestamp;
        self.message_interval = header.message_interval();
        self.state = State::WaitingForFollowUp;
        result = Ok(());
      },

      (&State::Uninitialized, GPTPMesage::FollowUp { .. }) =>
      {
        // We don't care about this case.
        result = Ok(());
      }

      (&State::WaitingForSync, GPTPMesage::Sync1Step { header, .. }) =>
      {
        // FIX: for 1 step there's no follow up.
        // NOTE: sync1 -> sync1 -> sync1 …
        self.state = State::WaitingForSync;

        match is_on_time(
          self.last_message_timestamp,
          timestamp,
          self.message_interval,
          self.margin)
        {
          TimeResult::TooEarly(duration) =>
          {
            result = Err(format!("1 step sync came in {:.3}ms too early.", duration.as_micros() as f64 / 1_000.));
          },
          TimeResult::TooLate(duration) =>
          {
            result = Err(format!("1 step sync cane in {:.3}ms too late.", duration.as_micros() as f64 / 1_000.));
          },
          TimeResult::Ok =>
          {
            result = Ok(());
          }
        }

        self.message_interval = header.message_interval();
      },

      (&State::WaitingForSync, GPTPMesage::Sync2Step { header, .. }) =>
      {
        // NOTE: sync2 -> followup -> sync2 -> followup -> sync2 -> followup -> …
        self.state = State::WaitingForFollowUp;

        match is_on_time(
          self.last_message_timestamp,
          timestamp,
          self.message_interval,
          self.margin)
        {
          TimeResult::TooEarly(duration) =>
          {
            result = Err(format!("2 step sync came in {:.3}ms too early.", duration.as_micros() as f64 / 1_000.));
          },
          TimeResult::TooLate(duration) =>
          {
            result = Err(format!("2 step sync came in {:.3}ms too late.", duration.as_micros() as f64 / 1_000.));
          },
          TimeResult::Ok =>
          {
            result = Ok(());
          }
        }

        self.message_interval = header.message_interval();
      },

      (&State::WaitingForFollowUp, GPTPMesage::FollowUp { header }) =>
      {
        // NOTE: sync2 -> followup -> sync2 -> followup -> sync2 -> followup -> …
        self.state = State::WaitingForSync;

        match is_on_time(
          self.last_message_timestamp,
          timestamp,
          self.message_interval,
          self.margin)
        {
          TimeResult::TooEarly(duration) =>
          {
            result = Err(format!("follow up came in {:.3}ms too early.", duration.as_micros() as f64 / 1_000.));
          },
          TimeResult::TooLate(duration) =>
          {
            result = Err(format!("follow up cane in {:.3}ms too late.", duration.as_micros() as f64 / 1_000.));
          },
          TimeResult::Ok =>
          {
            result = Ok(());
          }
        }

        self.message_interval = header.message_interval();
      },

      // TODO: better error message 😆.
      (state, message) =>
      {
        todo!()
      }
    };

    self.last_message_timestamp = timestamp;

    return result;
  }
}
