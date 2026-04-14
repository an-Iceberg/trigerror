use std::fmt::Debug;
use crate::{protocols::gptp::{message_type::MessageType, message_types::{announce::Announce, follow_up::FollowUp, peer_delay_request::PeerDelayRequest, peer_delay_response::PeerDelayResponse, peer_delay_response_follow_up::PeerDelayResponseFollowUp, signaling::Signaling, sync1step::Sync1Step, sync2step::Sync2Step}}, utils::{Octet, bytes_to_u16}};
use super::header::Header;

// TODO: add reserved fields.
pub enum GPTPMessage
{
  Announce(Announce),
  FollowUp(FollowUp),
  PeerDelayRequest(PeerDelayRequest),
  PeerDelayResponse(PeerDelayResponse),
  PeerDelayResponseFollowUp(PeerDelayResponseFollowUp),
  Signaling(Signaling),
  Sync1Step(Sync1Step),
  Sync2Step(Sync2Step),
}

impl GPTPMessage
{
  pub fn new(message_type: MessageType, payload: &[u8]) -> Result<Self, String>
  {
    let header = Header::new(message_type, payload);
    if payload.len() < header.message_length() as usize
    {
      return Err(format!(
        "payload is not long enough. Is: {}, should: {}.",
        payload.len(),
        header.message_length(),
      ));
    }

    return match message_type
    {
      MessageType::Announce => Ok(GPTPMessage::Announce(Announce::new(payload))),
      MessageType::FollowUp => Ok(GPTPMessage::FollowUp(FollowUp::new(payload))),
      MessageType::PeerDelayRequest => Ok(GPTPMessage::PeerDelayRequest(PeerDelayRequest::new(payload))),
      MessageType::PeerDelayResponse => Ok(GPTPMessage::PeerDelayResponse(PeerDelayResponse::new(payload))),
      MessageType::PeerDelayResponseFollowUp => Ok(GPTPMessage::PeerDelayResponseFollowUp(PeerDelayResponseFollowUp::new(payload))),
      MessageType::Signaling => Ok(GPTPMessage::Signaling(Signaling::new(payload))),
      MessageType::Sync1Step => Ok(GPTPMessage::Sync1Step(Sync1Step::new(payload))),
      MessageType::Sync2Step => Ok(GPTPMessage::Sync2Step(Sync2Step::new(payload))),
    };
  }

  pub fn header(&self) -> Header
  {
    return match self
    {
      GPTPMessage::Announce(announce) => announce.header(),
      GPTPMessage::FollowUp(follow_up) => follow_up.header(),
      GPTPMessage::PeerDelayRequest(peer_delay_request) => peer_delay_request.header(),
      GPTPMessage::PeerDelayResponse(peer_delay_response) => peer_delay_response.header(),
      GPTPMessage::PeerDelayResponseFollowUp(peer_delay_response_follow_up) => peer_delay_response_follow_up.header(),
      GPTPMessage::Signaling(signaling) => signaling.header(),
      GPTPMessage::Sync1Step(sync1step) => sync1step.header(),
      GPTPMessage::Sync2Step(sync2step) => sync2step.header(),
    };
  }
}
