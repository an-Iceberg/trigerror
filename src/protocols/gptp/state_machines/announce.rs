use std::{fmt::Display, time::Duration};
use crate::{mac::MAC, protocols::gptp::{message_type::MessageType, message_types::announce::Announce, state_machines::{mac_validator::MACValidator, time_validator::TimeValidator}}};

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

/// State machine for verifying Announce messages.
#[derive(Default)]
pub struct AnnounceSM
{
  state: State,
  mac_validator: MACValidator,
  time_validator: TimeValidator,
}

impl AnnounceSM
{
  pub fn validate(
    &mut self,
    announce: Announce,
    current_message_timestamp: Duration,
    new_source_mac: MAC,
  ) -> Result<(), Vec<String>>
  {
    if self.state == State::Uninitialized
    {
      let _ = self.validate_state(announce.header().message_type());
      let _ = self.validate_timing(current_message_timestamp, announce.header().message_interval());
      let _ = self.validate_mac(new_source_mac);
      return Ok(());
    }

    let mut errors = vec![];

    if let Err(error) = self.validate_state(announce.header().message_type())
    { errors.push(error); }
    if let Err(error) = self.validate_timing(current_message_timestamp, announce.header().message_interval())
    { errors.push(error); }
    if let Err(error) = self.validate_mac(new_source_mac)
    { errors.push(error); }

    if errors.is_empty() { return Ok(()); }
    else { return Err(errors); }
  }

  fn validate_state(&mut self, message_type: MessageType) -> Result<(), String>
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

  fn validate_timing(
    &mut self,
    current_message_timestamp: Duration,
    new_message_interval: Duration
  ) -> Result<(), String>
  {
    return self.time_validator.validate(current_message_timestamp, new_message_interval);
  }

  fn validate_mac(&mut self, new_source_mac: MAC) -> Result<(), String>
  {
    return self.mac_validator.validate(new_source_mac);
  }
}
