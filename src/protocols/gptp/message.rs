use crate::{Octet, bytes_to_u16, protocols::gptp::message_type::MessageType};
use super::header::Header;

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
  PdelayReq
  {
    // TODO: header 11.4.2
    header: Header,
  },
  PdelayResp
  {
    // TODO: header 11.4.2
    header: Header,
    request_receipt_timestamp: [Octet; 10],
    requesting_port_identity: [Octet; 10],
  },
  PdelayRespFollowUp
  {
    // TODO: header 11.4.2
    header: Header,
    response_origin_timestamp: [Octet; 10],
    requesting_port_identity: [Octet; 10],
  }
}

impl GPTPMesage
{
  /// Takes the message type and the ethernet payload and constructs a PTP message.
  pub fn new(message_type: MessageType, payload: &[u8]) -> Self
  {
    return match message_type
    {
      MessageType::Sync => GPTPMesage::Sync1Step
      {
        header: Header::new(message_type, payload),
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
      },
      MessageType::PdelayReq => GPTPMesage::PdelayReq
      {
        header: Header::new(message_type, payload),
      },
      MessageType::PdelayResp => GPTPMesage::PdelayResp
      {
        header: Header::new(message_type, payload),
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
        ]
      },
      MessageType::FollowUp => GPTPMesage::FollowUp
      {
        header: Header::new(message_type, payload),
      },
      MessageType::PdelayRespFollowUp => GPTPMesage::PdelayRespFollowUp
      {
        header: Header::new(message_type, payload),
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
      },
      MessageType::Announce => GPTPMesage::Announce
      {
        header: Header::new(message_type, payload),
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
      },
      MessageType::Signaling => GPTPMesage::Signaling
      {
        header: Header::new(message_type, payload),
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
      },
    };
  }
}
