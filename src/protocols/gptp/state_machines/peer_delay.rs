use std::{fmt::Display, time::Duration};
use crate::{mac::MAC, protocols::gptp::{message_type::MessageType, message_types::{peer_delay_request::PeerDelayRequest, peer_delay_response::PeerDelayResponse, peer_delay_response_follow_up::PeerDelayResponseFollowUp}, state_machines::{mac_validator::MACValidator, time_validator::TimeValidator}}};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum State
{
  #[default]
  Uninitialized,
  WaitingForPeerDelayRequest,
  WaitingForPeerDelayResponse,
  WaitingForPeerDelayResponseFollowUp,
}

impl Display for State
{
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    return formatter.write_str(match self
    {
      State::Uninitialized => "Uninitialized",
      State::WaitingForPeerDelayRequest => "WaitingForPeerDelayRequest",
      State::WaitingForPeerDelayResponse => "WaitingForPerDelayResponse",
      State::WaitingForPeerDelayResponseFollowUp => "WaitingForPerDelayResponseFollowUp",
    });
  }
}

/// State machine for verifying PeerDelay messages.
#[derive(Default)]
pub struct PeerDelaySM
{
  state: State,
  requester_mac_validator: MACValidator,
  responder_mac_validator: MACValidator,
  time_validator: TimeValidator,
}

// TODO: initialize the state machine without causing errors.
impl PeerDelaySM
{
  pub fn validate_peer_delay_request(
    &mut self,
    peer_delay_request: PeerDelayRequest,
    current_message_timestamp: Duration,
    new_source_mac: MAC,
  ) -> Result<(), Vec<String>>
  {
    // Initializing the state machine. (We just ignore the first errors)
    if matches!(self.state, State::Uninitialized)
    {
      let _ = self.validate_state(MessageType::PeerDelayRequest);
      let _ = self.validate_timing(
        current_message_timestamp,
        peer_delay_request.header().message_interval(),
        MessageType::PeerDelayRequest,
      );
      let _ = self.validate_request_mac(new_source_mac, MessageType::PeerDelayRequest);
      return Ok(());
    }

    let mut errors = vec![];

    if let Err(error) = self.validate_state(MessageType::PeerDelayRequest)
    { errors.push(error); }
    if let Err(error) = self.validate_timing(
      current_message_timestamp,
      peer_delay_request.header().message_interval(),
      MessageType::PeerDelayRequest,
    )
    { errors.push(error); }
    if let Err(error) = self.validate_request_mac(new_source_mac, MessageType::PeerDelayRequest)
    { errors.push(error); }

    if errors.is_empty() { return Ok(()); }
    else { return Err(errors); }
  }

  pub fn validate_peer_delay_response(
    &mut self,
    peer_delay_response: PeerDelayResponse,
    current_message_timestamp: Duration,
    new_source_mac: MAC,
  ) -> Result<(), Vec<String>>
  {
    let mut errors = vec![];

    if let Err(error) = self.validate_state(MessageType::PeerDelayResponse)
    { errors.push(error); }
    if let Err(error) = self.validate_timing(
      current_message_timestamp,
      peer_delay_response.header().message_interval(),
      MessageType::PeerDelayResponse,
    )
    { errors.push(error); }
    // A really ugly way to correctly initialize the response MAC of the state machine.
    // If response MAC is uninitialized, ignore generated error. Else return it.
    if self.responder_mac_validator.mac() == MAC::from_bytes((00, 00, 00, 00, 00, 00))
    { let _ = self.validate_response_mac(new_source_mac, MessageType::PeerDelayResponse); }
    else if let Err(error) = self.validate_response_mac(new_source_mac, MessageType::PeerDelayResponse)
    { errors.push(error); }

    if errors.is_empty() { return Ok(()); }
    else { return Err(errors); }
  }

  pub fn validate_peer_delay_response_follow_up(
    &mut self,
    peer_delay_response_follow_up: PeerDelayResponseFollowUp,
    current_message_timestamp: Duration,
    new_source_mac: MAC,
  ) -> Result<(), Vec<String>>
  {
    let mut errors = vec![];

    if let Err(error) = self.validate_state(MessageType::PeerDelayResponseFollowUp)
    { errors.push(error); }
    if let Err(error) = self.validate_timing(
      current_message_timestamp,
      peer_delay_response_follow_up.header().message_interval(),
      MessageType::PeerDelayResponseFollowUp,
    )
    { errors.push(error); }
    if let Err(error) = self.validate_response_mac(new_source_mac, MessageType::PeerDelayResponseFollowUp)
    { errors.push(error); }

    if errors.is_empty() { return Ok(()); }
    else { return Err(errors); }
  }

  fn validate_state(&mut self, message_type: MessageType) -> Result<(), String>
  {
    use State::{Uninitialized, WaitingForPeerDelayRequest, WaitingForPeerDelayResponse, WaitingForPeerDelayResponseFollowUp};
    use MessageType::{PeerDelayRequest, PeerDelayResponse, PeerDelayResponseFollowUp};

    return match (self.state, message_type)
    {
      // TODO: how do we exit the uninitialized state?
      // Init
      (Uninitialized, PeerDelayRequest) => { self.state = WaitingForPeerDelayResponse; Ok(()) }

      // Expected state changes.
      (WaitingForPeerDelayRequest, PeerDelayRequest) => { self.state = WaitingForPeerDelayResponse; Ok(()) }
      (WaitingForPeerDelayResponse, PeerDelayResponse) => { self.state = WaitingForPeerDelayResponseFollowUp; Ok(()) }
      (WaitingForPeerDelayResponseFollowUp, PeerDelayResponseFollowUp) => { self.state = WaitingForPeerDelayRequest; Ok(()) }

      // Unexpected state changes.
      (WaitingForPeerDelayRequest, PeerDelayResponse) =>
      {
        self.state = WaitingForPeerDelayResponseFollowUp;
        Err("Waiting for PeerDelayRequest but got PeerDelayResponse".to_string())
      }
      (WaitingForPeerDelayRequest, PeerDelayResponseFollowUp) =>
        Err("Waiting for PeerDelayRequest but got PeerDelayResponseFollowUp".to_string()),

      (WaitingForPeerDelayResponse, PeerDelayResponseFollowUp) =>
      {
        self.state = WaitingForPeerDelayRequest;
        Err("Waiting for PeerDelayResponse but got PeerDelayResponseFollowUp".to_string())
      }
      (WaitingForPeerDelayResponse, PeerDelayRequest) =>
        Err("Waiting for PeerDelayResponse but got PeerDelayRequest".to_string()),
      (WaitingForPeerDelayResponseFollowUp, PeerDelayRequest) =>
      {
        self.state = WaitingForPeerDelayResponse;
        Err("Waiting for PeerDelayResponseFollowUp but got PeerDelayRequest".to_string())
      }
      (WaitingForPeerDelayResponseFollowUp, PeerDelayResponse) =>
        Err("Waiting for PeerDelayResponseFollowUp but got PeerDelayResponse".to_string()),

      // Catchall
      (state, message_type) => Err(format!(
        "Unknown state and message combination from PeerDelaySM: state: {state}, message type: {message_type}"
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

  fn validate_request_mac(&mut self, new_source_mac: MAC, message_type: MessageType) -> Result<(), String>
  {
    return self.requester_mac_validator.validate(new_source_mac, message_type);
  }

  fn validate_response_mac(&mut self, new_source_mac: MAC, message_type: MessageType) -> Result<(), String>
  {
    return self.responder_mac_validator.validate(new_source_mac, message_type);
  }
}
