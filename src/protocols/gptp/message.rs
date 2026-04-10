use std::fmt::Debug;

use crate::{Octet, bytes_to_u16, protocols::gptp::{message_type::MessageType}};
use super::header::Header;

// TODO: reverse byte order.

// TODO: debug print as hex numbers.

// TODO: add reserved fields.

#[derive(Debug)]
pub enum GPTPMesage
{
  Announce
  {
    header: Header,
    current_utc_offset: u16,
    grandmaster_priority_1: Octet,
    grandmaster_clock_quality: [Octet; 4],
    grandmaster_priority_2: Octet,
    grandmaster_identity: [Octet; 8],
    steps_removed: u16,
    time_source: Octet,
    tlv_type: [Octet; 2],
    length_field: [Octet; 2],
    path_sequence: Vec<Octet>,
  },
  Signaling
  {
    header: Header,
    target_port_identity: [Octet; 10],
    // TODO: implement last field
  },
  Sync1Step
  {
    // TODO: header 11.4.2
    header: Header,
    origin_timestamp: [Octet; 10],
    tlv_type: u16,
    length_field: u16,
    organization_id: [Octet; 3],
    organization_sub_type: [Octet; 3],
    cumulative_scaled_rate_offset: [Octet; 4],
    gm_time_base_indicator: [Octet; 2],
    last_gm_phase_change: [Octet; 12],
    scaled_last_gm_frequency_change: [Octet; 4],
  },
  Sync2Step
  {
    // TODO: header 11.4.2
    header: Header,
    reserved: [Octet; 10]
  },
  FollowUp
  {
    // TODO: header 11.4.2
    header: Header,
    // TODO: different versions
    // TODO: 11.4.4.1-3
  },
  PeerDelayRequest
  {
    // TODO: header 11.4.2
    header: Header,
  },
  PeerDelayResponse
  {
    // TODO: header 11.4.2
    header: Header,
    request_receipt_timestamp: [Octet; 10],
    requesting_port_identity: [Octet; 10],
  },
  PeerDelayResponseFollowUp
  {
    // TODO: header 11.4.2
    header: Header,
    response_origin_timestamp: [Octet; 10],
    requesting_port_identity: [Octet; 10],
  }
}

impl GPTPMesage
{
  pub fn header(&self) -> &Header
  {
    return match self
    {
        GPTPMesage::Announce { header, .. } => header,
        GPTPMesage::Signaling { header, .. } => header,
        GPTPMesage::Sync1Step { header, .. } => header,
        GPTPMesage::Sync2Step { header, .. } => header,
        GPTPMesage::FollowUp { header } => header,
        GPTPMesage::PeerDelayRequest { header } => header,
        GPTPMesage::PeerDelayResponse { header, .. } => header,
        GPTPMesage::PeerDelayResponseFollowUp { header, .. } => header,
    };
  }

  // FIX: check that the payload is long enough! Some packets don't seem to be long enough.
  /// Takes the message type and the ethernet payload and constructs a PTP message.
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
      MessageType::Sync1Step => Ok(Self::new_sync_1_step(header, payload)),
      MessageType::Sync2Step => Ok(Self::new_sync_2_step(header, payload)),
      MessageType::PeerDelayRequest => Ok(Self::new_peer_delay_request(header, payload)),
      MessageType::PeerDelayResponse => Ok(Self::new_peer_delay_response(header, payload)),
      MessageType::FollowUp => Ok(Self::new_follow_up(header, payload)),
      MessageType::PeerDelayResponseFollowUp => Ok(Self::new_peer_delay_response_follow_up(header, payload)),
      MessageType::Announce => Ok(Self::new_announce(header, payload)),
      MessageType::Signaling => Ok(Self::new_signaling(header, payload)),
    };
  }

  fn new_announce(header: Header, payload: &[u8]) -> Self
  {
    return GPTPMesage::Announce
    {
      header,
      current_utc_offset: bytes_to_u16(payload[44], payload[45]),
      grandmaster_priority_1: payload[47],
      grandmaster_clock_quality: [
        payload[48],
        payload[49],
        payload[50],
        payload[51],
      ],
      grandmaster_priority_2: payload[52],
      grandmaster_identity: [
        payload[53],
        payload[54],
        payload[55],
        payload[56],
        payload[57],
        payload[58],
        payload[59],
        payload[60],
      ],
      steps_removed: bytes_to_u16(payload[61], payload[62]),
      time_source: payload[63],
      tlv_type: [
        payload[64],
        payload[65],
      ],
      length_field: [
        payload[66],
        payload[67],
      ],
      path_sequence: payload[68..].into(),
    };
  }

  fn new_signaling(header: Header, payload: &[u8]) -> Self
  {
    return GPTPMesage::Signaling
    {
      header,
      // TODO: this should be all 1s.
      // TODO: verify that payload also has all 1s.
      target_port_identity: [
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
        0b1111_1111,
      ],
      // TODO: TLV table 10-20
      // TODO: only gPTP-capable TLV
    };
  }

  fn new_sync_1_step(header: Header, payload: &[u8]) -> Self
  {
    return GPTPMesage::Sync1Step
    {
      header,
      origin_timestamp: [
        payload[34],
        payload[35],
        payload[36],
        payload[37],
        payload[38],
        payload[39],
        payload[40],
        payload[41],
        payload[42],
        payload[43],
      ],
      tlv_type: bytes_to_u16(payload[44], payload[45]),
      length_field: bytes_to_u16(payload[46], payload[47]),
      organization_id: [
        payload[48],
        payload[49],
        payload[50],
      ],
      organization_sub_type: [
        payload[51],
        payload[52],
        payload[53],
      ],
      cumulative_scaled_rate_offset: [
        payload[54],
        payload[55],
        payload[56],
        payload[57],
      ],
      gm_time_base_indicator: [
        payload[58],
        payload[59],
      ],
      last_gm_phase_change: [
        payload[60],
        payload[61],
        payload[62],
        payload[63],
        payload[64],
        payload[65],
        payload[66],
        payload[67],
        payload[68],
        payload[69],
        payload[70],
        payload[71],
      ],
      scaled_last_gm_frequency_change: [
        payload[72],
        payload[73],
        payload[74],
        payload[75],
      ],
    };
  }

  fn new_sync_2_step(header: Header, payload: &[u8]) -> Self
  {
    return GPTPMesage::Sync2Step
    {
      header,
      reserved: [
        payload[34],
        payload[35],
        payload[36],
        payload[37],
        payload[38],
        payload[39],
        payload[40],
        payload[41],
        payload[42],
        payload[43],
      ]
    };
  }

  fn new_follow_up(header: Header, payload: &[u8]) -> Self
  { return GPTPMesage::FollowUp { header }; }

  fn new_peer_delay_request(header: Header, payload: &[u8]) -> Self
  { return GPTPMesage::PeerDelayRequest { header }; }

  fn new_peer_delay_response(header: Header, payload: &[u8]) -> Self
  {
    return GPTPMesage::PeerDelayResponse
    {
      header,
      request_receipt_timestamp: [
        payload[34],
        payload[35],
        payload[36],
        payload[37],
        payload[38],
        payload[39],
        payload[40],
        payload[41],
        payload[42],
        payload[43],
      ],
      requesting_port_identity: [
        payload[44],
        payload[45],
        payload[46],
        payload[47],
        payload[48],
        payload[49],
        payload[50],
        payload[51],
        payload[52],
        payload[53],
      ],
    };
  }

  fn new_peer_delay_response_follow_up(header: Header, payload: &[u8]) -> Self
  {
    return GPTPMesage::PeerDelayResponseFollowUp
    {
      header,
      response_origin_timestamp: [
        payload[34],
        payload[35],
        payload[36],
        payload[37],
        payload[38],
        payload[39],
        payload[40],
        payload[41],
        payload[42],
        payload[43],
      ],
      requesting_port_identity: [
        payload[44],
        payload[45],
        payload[46],
        payload[47],
        payload[48],
        payload[49],
        payload[50],
        payload[51],
        payload[52],
        payload[53],
      ],
    };
  }
}
