pub mod flags;
pub mod header;
pub mod message;
pub mod message_type;
pub mod message_types;
pub mod state_machines;

use std::collections::HashMap;
use pcap_file::pcap::PcapPacket;
use crate::{constants::PTP_ETHER_TYPE, mac::MAC, protocols::{Protocol, gptp::{message::{GPTPMessage}, message_type::MessageType, state_machines::{announce::AnnounceSM, peer_delay::PeerDelaySM, signaling::SignalingSM, sync::SyncSM}}}, utils::{bytes_to_u16, get_bit}};

#[derive(Default)]
pub struct GPTP
{
  sync_sm: SyncSM,
  announce_sm: AnnounceSM,
  peer_delay_sm: PeerDelaySM,
  signaling_sm: SignalingSM,

  domains: HashMap<u8, Domain>,
}

#[derive(Default)]
struct Domain
{
  sync_sm: SyncSM,
  announce_sm: AnnounceSM,
  peer_delay_sm: PeerDelaySM,
  signaling_sm: SignalingSM,
}

impl GPTP
{
  pub fn new() -> Self
  {
    return GPTP
    {
      sync_sm: SyncSM::default(),
      announce_sm: AnnounceSM::default(),
      peer_delay_sm: PeerDelaySM::default(),
      signaling_sm: SignalingSM::default(),
      domains: HashMap::with_capacity(127),
    };
  }
}

impl Protocol for GPTP
{
  fn validate_packet(&mut self, packet: &PcapPacket) -> Result<(), Vec<String>>
  {
    // PTP = 0x88f7

    // Not PTP; we don't care.
    if bytes_to_u16(packet.data[12], packet.data[13]) != PTP_ETHER_TYPE
    { return Ok(()); }

    let ether_source = MAC::from_bytes((
      packet.data[6],
      packet.data[7],
      packet.data[8],
      packet.data[9],
      packet.data[10],
      packet.data[11],
    ));

    let payload = &packet.data[14..];

    let message_type = match MessageType::from_u8(payload[0] & 0b0000_1111, get_bit(payload[6], 1))
    {
      Ok(message_type) => message_type,
      Err(error) => return Err(vec![error]),
    };

    let message = match GPTPMessage::new(message_type, payload)
    {
      Ok(message) => message,
      Err(error) => return Err(vec![error]),
    };

    return match message
    {
      GPTPMessage::Announce(announce) =>
        self.announce_sm.validate(announce, packet.timestamp, ether_source),
      GPTPMessage::Signaling(signaling) =>
        self.signaling_sm.validate(signaling, packet.timestamp, ether_source),
      GPTPMessage::PeerDelayRequest(peer_delay_request) =>
        self.peer_delay_sm.validate_peer_delay_request(peer_delay_request, packet.timestamp, ether_source),
      GPTPMessage::PeerDelayResponse(peer_delay_response) =>
        self.peer_delay_sm.validate_peer_delay_response(peer_delay_response, packet.timestamp, ether_source),
      GPTPMessage::PeerDelayResponseFollowUp(peer_delay_response_follow_up) =>
        self.peer_delay_sm.validate_peer_delay_response_follow_up(peer_delay_response_follow_up, packet.timestamp, ether_source),
      GPTPMessage::FollowUp(follow_up) =>
        self.sync_sm.validate_follow_up(follow_up, ether_source),
      GPTPMessage::Sync1Step(sync_1_step) =>
        self.sync_sm.validate_sync_1_step(sync_1_step, packet.timestamp, ether_source),
      GPTPMessage::Sync2Step(sync_2_step) =>
        self.sync_sm.validate_sync_2_step(sync_2_step, packet.timestamp, ether_source),
    };
  }
}
