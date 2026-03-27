use super::{Octet, message_type::MessageType, flags::Flags};

pub struct Header
{
  major_s_dold: Octet,
  message_type: MessageType,
  minor_version_ptp: Octet,
  version_ptp: Octet,
  message_length: u16,
  domain_number: Octet,
  minor_s_dold: Octet,
  flags: Flags,
  correction_field: [Octet; 8],
  message_type_specific: [Octet; 4],
  source_port_identity: [Octet; 10],
  sequence_id: [Octet; 2],
  control_field: Octet,
  log_message_interval: Octet,
}
