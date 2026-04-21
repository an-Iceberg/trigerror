use std::{fmt::Display, time::Duration};
use crate::{mac::MAC, protocols::gptp::{message::GPTPMessage, message_type::MessageType, message_types::{announce::Announce, follow_up::FollowUp, sync1step::Sync1Step, sync2step::Sync2Step}, state_machines::{mac_validator::MACValidator, time_validator::TimeValidator}}};

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

/// State machine for verifying Sync messages.
///
/// Adopted from Figure 11-6.
#[derive(Default)]
pub struct SyncSM
{
  state: State,
  mac_validator: MACValidator,
  time_validator: TimeValidator,
}

impl SyncSM
{
  // pub fn validate(
  //   &mut self,
  //   message: GPTPMessage,
  //   current_message_timestamp: Duration,
  //   new_source_mac: MAC,
  // ) -> Result<(), Vec<String>>
  // {
  //   if self.state == State::Uninitialized
  //   {
  //     let _ = self.validate_state(message.header().message_type());
  //     let _ = self.validate_timing(current_message_timestamp, message.header().message_interval());
  //     let _ = self.validate_mac(new_source_mac);
  //     return Ok(());
  //   }

  //   let mut errors = vec![];

  //   if let Err(error) = self.validate_state(message.header().message_type())
  //   { errors.push(error); }
  //   if let Err(error) = self.validate_timing(current_message_timestamp, message.header().message_interval())
  //   { errors.push(error); }
  //   if let Err(error) = self.validate_mac(new_source_mac)
  //   { errors.push(error); }

  //   if errors.is_empty() { return Ok(()); }
  //   else { return Err(errors); }
  // }

  pub fn validate_follow_up(
    &mut self,
    follow_up: FollowUp,
    new_source_mac: MAC,
  ) -> Result<(), Vec<String>>
  {
    if self.state == State::Uninitialized
    { return Ok(()); }

    let mut errors = vec![];

    if let Err(error) = self.validate_state(follow_up.header().message_type())
    { errors.push(error); }
    if let Err(error) = self.validate_mac(new_source_mac)
    { errors.push(error); }

    if errors.is_empty() { return Ok(()); }
    else { return Err(errors); }
  }

  pub fn validate_sync_1_step(
    &mut self,
    sync_1_step: Sync1Step,
    current_message_timestamp: Duration,
    new_source_mac: MAC,
  ) -> Result<(), Vec<String>>
  {
    if self.state == State::Uninitialized
    {
      let _ = self.validate_state(sync_1_step.header().message_type());
      let _ = self.validate_timing(current_message_timestamp, sync_1_step.header().message_interval(), MessageType::Sync1Step);
      let _ = self.validate_mac(new_source_mac);
      return Ok(());
    }

    let mut errors = vec![];

    if let Err(error) = self.validate_state(sync_1_step.header().message_type())
    { errors.push(error); }
    if let Err(error) = self.validate_timing(current_message_timestamp, sync_1_step.header().message_interval(), MessageType::Sync1Step)
    { errors.push(error); }
    if let Err(error) = self.validate_mac(new_source_mac)
    { errors.push(error); }

    if errors.is_empty() { return Ok(()); }
    else { return Err(errors); }
  }

  pub fn validate_sync_2_step(
    &mut self,
    sync_2_step: Sync2Step,
    current_message_timestamp: Duration,
    new_source_mac: MAC,
  ) -> Result<(), Vec<String>>
  {
    if self.state == State::Uninitialized
    {
      let _ = self.validate_state(sync_2_step.header().message_type());
      let _ = self.validate_timing(current_message_timestamp, sync_2_step.header().message_interval(), MessageType::Sync2Step);
      let _ = self.validate_mac(new_source_mac);
      return Ok(());
    }

    let mut errors = vec![];

    if let Err(error) = self.validate_state(sync_2_step.header().message_type())
    { errors.push(error); }
    if let Err(error) = self.validate_timing(current_message_timestamp, sync_2_step.header().message_interval(), MessageType::Sync2Step)
    { errors.push(error); }
    if let Err(error) = self.validate_mac(new_source_mac)
    { errors.push(error); }

    if errors.is_empty() { return Ok(()); }
    else { return Err(errors); }
  }

  fn validate_state(&mut self, message_type: MessageType) -> Result<(), String>
  {
    // INFO: in Rust it is a generally discouraged to import enum variants by name b/c hidden
    // INFO: name space contamination could happen easily which could lead to footguns. Here it is
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

  fn validate_timing(
    &mut self,
    current_message_timestamp: Duration,
    new_message_interval: Duration,
    message_type: MessageType,
  ) -> Result<(), String>
  {
    return self.time_validator.validate(current_message_timestamp, new_message_interval, message_type);
  }

  fn validate_mac(&mut self, new_source_mac: MAC) -> Result<(), String>
  {
    return self.mac_validator.validate(new_source_mac);
  }
}
