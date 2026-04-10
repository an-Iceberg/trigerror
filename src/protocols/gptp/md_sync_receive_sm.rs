use std::{fmt::Display, time::Duration};
use crate::{duration_to_string, is_on_time, protocols::gptp::{message::GPTPMesage, message_type::MessageType}};

#[derive(Debug, Default, Clone, Copy)]
enum State
{
  WaitingForFollowUp,
  WaitingForSync1Step,
  WaitingForSync2Step,
  #[default]
  Uninitialized,
}

impl Display for State
{
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    return formatter.write_str(match self
    {
      State::WaitingForFollowUp => "WaitingForFollowUp",
      Self::WaitingForSync1Step => "WaitingFor1StepSync",
      Self::WaitingForSync2Step => "WaitingFor2StepSync",
      State::Uninitialized => "Uninitialized",
    });
  }
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

pub struct MDSyncReceiveStateMachine
{
  state: State,

  message_interval: Duration,
  last_message_timestamp: Duration,
  margin: f64,
}

impl Default for MDSyncReceiveStateMachine
{
  fn default() -> Self
  {
    return MDSyncReceiveStateMachine
    {
      state: State::default(),
      message_interval: Duration::default(),
      last_message_timestamp: Duration::default(),
      margin: 0.3,
    };
  }
}

impl MDSyncReceiveStateMachine
{
  pub fn new() -> Self { return Default::default(); }

  pub fn validate_state(&mut self, message_type: MessageType) -> Result<(), String>
  {
    // NOTE: in Rust it is a generally discouraged to import enum variants by name b/c
    // NOTE: name space pollution could happen easily which could lead to footguns. Here it is
    // NOTE: used so that the state transitions are more easy to read.
    use State::{Uninitialized, WaitingForSync1Step, WaitingForSync2Step, WaitingForFollowUp};
    use MessageType::{Sync1Step, Sync2Step, FollowUp};

    /*
    There are 2 lanes:
    1StepSync -> 1StepSync -> 1StepSync -> …
    or
    2StepSync -> FollowUp -> 2StepSync -> FollowUp -> 2StepSync -> FollowUp -> …
    Changing from one lane to the other is an error. But if such a thing happens then we do
    update the state to that new lane.
    TODO: ask about FollowUp -> 1StepSync
    TODO: do we need to keep track of which lane we're in?
    */

    // This is ugly b/c the protocol has a weird structure. In the message type field there's no
    // distinction between 1-Step Sync and 2-Step Sync.
    return match (self.state, message_type)
    {
      // Expected state changes.
      (Uninitialized, FollowUp) => { Ok(()) } // We don't care about this case.
      (Uninitialized, Sync1Step) => { self.state = WaitingForSync1Step; Ok(()) }
      (Uninitialized, Sync2Step) => { self.state = WaitingForFollowUp; Ok(()) }
      (WaitingForSync1Step, Sync1Step) => { Ok(()) }
      (WaitingForSync2Step, Sync2Step) => { self.state = WaitingForFollowUp; Ok(()) }
      (WaitingForFollowUp, FollowUp) => { self.state = WaitingForSync2Step; Ok(()) }

      // Unexpected state changes.
      (WaitingForSync1Step, FollowUp) =>
      {
        self.state = WaitingForSync2Step;
        Err("Waiting for Sync1Step but got FollowUp".to_string())
      }
      (WaitingForSync1Step, Sync2Step) =>
      {
        self.state = WaitingForFollowUp;
        Err("Waiting for Sync1Step but got Sync2Step".to_string())
      }
      (WaitingForSync2Step, FollowUp) =>
      {
        Err("Waiting for Sync2Step but got FollowUp".to_string())
      }
      (WaitingForSync2Step, Sync1Step) =>
      {
        self.state = WaitingForSync1Step;
        Err("Waiting for Sync2Step but got Sync1Step".to_string())
      }

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
