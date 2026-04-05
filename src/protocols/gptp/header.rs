use std::time::Duration;
use crate::{Octet, bytes_to_u16, protocols::gptp::flags::Flags};
use super::message_type::MessageType;

/// Represents the header of a gPTP message as defined in the standard (802.1AS-2025) 11.4.2.
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

  pub fn message_type(&self) -> &MessageType { return &self.message_type; }
  pub fn major_sd_old(&self) -> Octet { return self.major_sd_old; }
  pub fn version_ptp(&self) -> Octet { return self.version_ptp; }
  pub fn minor_version_ptp(&self) -> Octet { return self.minor_version_ptp; }
  pub fn message_length(&self) -> u16 { return self.message_length; }
  pub fn domain_number(&self) -> Octet { return self.domain_number; }
  pub fn minor_sd_old(&self) -> Octet { return self.minor_sd_old; }
  pub fn flags(&self) -> &Flags { return &self.flags; }
  pub fn correction_field(&self) -> &[Octet; 8] { return &self.correction_field; }
  pub fn message_type_specific(&self) -> &[Octet; 4] { return &self.message_type_specific; }
  pub fn source_port_identity(&self) -> &[Octet; 10] { return &self.source_port_identity; }
  pub fn sequence_id(&self) -> &[Octet; 2] { return &self.sequence_id; }
  pub fn control_field(&self) -> Octet { return self.control_field; }

  /// From 10.3.10.7:
  /// > The current value of the logarithm of base 2 of the mean time interval \[…].
  pub fn log_message_interval(&self) -> Octet { return self.log_message_interval; }

  /// Returns the expected time interval until the next message as a `std::time::Duration`.
  pub fn message_interval(&self) -> Duration
  { return Duration::from_secs((self.log_message_interval as u64).pow(2)); }
}
