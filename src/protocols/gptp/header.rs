use crate::{Octet, bytes_to_u16, protocols::gptp::flags::Flags};
use super::message_type::MessageType;

pub struct Header
{
  message_type: MessageType,
  major_sd_old: Octet,
  version_ptp: Octet,
  minor_version_ptp: Octet,
  message_length: u16,
  domain_number: Octet,
  minor_sd_old: Octet,
  flags: Flags,
  correction_field: [Octet; 8],
  message_type_specific: [Octet; 4],
  source_port_identity: [Octet; 10],
  sequence_id: [Octet; 2],
  control_field: Octet,
  log_message_interval: Octet,
}

impl Header
{
  pub fn new(message_type: MessageType, payload: &[u8]) -> Self
  {
    return Header
    {
      message_type,
      // We only care about the last 4 bits.
      major_sd_old: payload[0] & 0b0000_1111,
      // We only care about the first 4 bits.
      version_ptp: payload[1] & 0b1111_0000,
      minor_version_ptp: payload[1] & 0b0000_1111,
      message_length: bytes_to_u16(payload[2], payload[3]),
      domain_number: payload[4],
      minor_sd_old: payload[5],
      flags: Flags::new(payload[6], payload[7]),
      correction_field: [
        payload[8],
        payload[9],
        payload[10],
        payload[11],
        payload[12],
        payload[13],
        payload[14],
        payload[15],
      ],
      message_type_specific: [
        payload[16],
        payload[17],
        payload[18],
        payload[19],
      ],
      source_port_identity: [
        payload[20],
        payload[21],
        payload[22],
        payload[23],
        payload[24],
        payload[25],
        payload[26],
        payload[27],
        payload[28],
        payload[29],
      ],
      sequence_id: [
        payload[30],
        payload[31],
      ],
      control_field: payload[32],
      log_message_interval: payload[33],
    };
  }
}
