use crate::{protocols::gptp::{header::Header, message_type::MessageType}, utils::{Octet, bytes_to_u16}};

pub struct Announce
{
  header: Header,
  reserved: [Octet; 10],
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
}

impl Announce
{
  pub fn header(&self) -> Header
  { return self.header; }

  pub fn new(payload: &[u8]) -> Self
  {
    return Self
    {
      header: Header::new(MessageType::Announce, payload),
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
      ],
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
}
