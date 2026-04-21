use std::{fmt::Display, time::Duration};
use crate::{mac::MAC, protocols::gptp::{message_type::MessageType, message_types::signaling::Signaling, state_machines::{mac_validator::MACValidator, time_validator::TimeValidator}}};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum State
{
  #[default]
  Uninitialized,
  WaitingForSignaling,
}

impl Display for State
{
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    return formatter.write_str(match self
    {
      State::Uninitialized => "Uninitialized",
      State::WaitingForSignaling => "WaitingForSignaling",
    });
  }
}

/// State machine for verifying Signaling messages.
#[derive(Default)]
pub struct SignalingSM
{
  state: State,
  mac_validator: MACValidator,
  time_validator: TimeValidator,
}

// TODO: log_message_interval == 127 is to be ignored. (No time validation).

impl SignalingSM
{
  pub fn validate(
    &mut self,
    signaling: Signaling,
    current_message_timestamp: Duration,
    new_source_mac: MAC,
  ) -> Result<(), Vec<String>>
  {
    if matches!(self.state, State::Uninitialized)
    {
      let _ = self.validate_state(MessageType::Signaling);
      if signaling.header().log_message_interval() != 127
      { let _ = self.validate_timing(current_message_timestamp, signaling.header().message_interval()); }
      let _ = self.validate_mac(new_source_mac, MessageType::Signaling);
      return Ok(());
    }

    let mut errors = vec![];

    if let Err(error) = self.validate_state(MessageType::Signaling)
    { errors.push(error); }
    if
      signaling.header().log_message_interval() != 127 &&
      let Err(error) = self.validate_timing(current_message_timestamp, signaling.header().message_interval())
    { errors.push(error); }
    if let Err(error) = self.validate_mac(new_source_mac, MessageType::Signaling)
    { errors.push(error); }

    if errors.is_empty() { return Ok(()); }
    else { return Err(errors); }
  }

  fn validate_state(&mut self, message_type: MessageType) -> Result<(), String>
  {
    use State::{Uninitialized, WaitingForSignaling};
    use MessageType::Signaling;

    return match (self.state, message_type)
    {
      // Init
      (Uninitialized, Signaling) => { self.state = WaitingForSignaling; Ok(()) }

      // Expected state changes.
      (WaitingForSignaling, Signaling) => { Ok(()) }

      // Unexpected state changes.

      // Catchall
      (state, message_type) => Err(format!(
        "Unknown state and message combination from SignalingSM: state: {state}, message type: {message_type}"
      ))
    };
  }

  fn validate_timing(
    &mut self,
    current_message_timestamp: Duration,
    new_message_interval: Duration,
  ) -> Result<(), String>
  {
    return self.time_validator.validate(current_message_timestamp, new_message_interval, MessageType::Signaling);
  }

  fn validate_mac(&mut self, new_source_mac: MAC, message_type: MessageType) -> Result<(), String>
  {
    return self.mac_validator.validate(new_source_mac, message_type);
  }
}
