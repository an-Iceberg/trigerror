use std::{fmt::Display, time::Duration};
use crate::{mac::MAC, protocols::gptp::message_type::MessageType, utils::duration_to_string};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum State
{
  #[default]
  Uninitialized,
  WaitingForFollowUp,
  WaitingForSync1Step,
  WaitingForSync2Step,
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

// Figure 11-6

pub struct SyncStateMachine
{
  source_mac: MAC,
  state: State,
  message_interval: Duration,
  last_message_timestamp: Duration,
  margin: f64,
}

impl Default for SyncStateMachine
{
  fn default() -> Self
  {
    return SyncStateMachine
    {
      source_mac: MAC::default(),
      state: State::default(),
      message_interval: Duration::default(),
      last_message_timestamp: Duration::default(),
      margin: 0.3,
    };
  }
}

impl SyncStateMachine
{
  pub fn new() -> Self { return Default::default(); }

  pub fn is_uninitialized(&self) -> bool { return self.state == State::Uninitialized; }

  pub fn validate_state(&mut self, message_type: MessageType) -> Result<(), String>
  {
    // INFO: in Rust it is a generally discouraged to import enum variants by name b/c
    // INFO: name space pollution could happen easily which could lead to footguns. Here it is
    // INFO: used so that the state transitions are more easy to read.
    use State::{Uninitialized, WaitingForSync1Step, WaitingForSync2Step, WaitingForFollowUp};
    use MessageType::{Sync1Step, Sync2Step, FollowUp};

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
        { Err("Waiting for Sync2Step but got FollowUp".to_string()) }
      (WaitingForSync2Step, Sync1Step) =>
      {
        self.state = WaitingForSync1Step;
        Err("Waiting for Sync2Step but got Sync1Step".to_string())
      }
      (WaitingForFollowUp, Sync2Step) =>
        { Err("Waiting for FollowUp but got Sync2Step".to_string()) }
      (WaitingForFollowUp, Sync1Step) =>
      {
        self.state = WaitingForSync1Step;
        Err("Waiting for FollowUp but got Sync1Step".to_string())
      }

      // Catchall
      (state, message_type) => Err(format!(
        "Unknown state and message combination from SyncSM: state: {state}, message type: {message_type:?}"
      ))
    };
  }

  // TODO: implement this in a time driven (instead of an event driven) way.
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

    // NOTE: Aurel's suggestion but it produces the same results.
    // let should_time = self.last_message_timestamp + self.message_interval;
    // let margin = self.message_interval.mul_f64(self.margin);
    // let lower_bound = should_time - margin;
    // let upper_bound = should_time + margin;

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
