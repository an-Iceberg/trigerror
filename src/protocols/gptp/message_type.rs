use std::fmt::{Debug, Display};

#[derive(Clone, Copy)]
pub enum MessageType
{
  Sync1Step,
  Sync2Step,
  PeerDelayRequest,
  PeerDelayResponse,
  FollowUp,
  PeerDelayResponseFollowUp,
  Announce,
  Signaling,
}

impl MessageType
{
  pub fn from_u8(byte: u8, two_step: bool) -> Result<MessageType, String>
  {
    return match byte
    {
      0x0 =>
        if two_step { Ok(MessageType::Sync2Step) }
        else { Ok(MessageType::Sync1Step) },
      0x2 => Ok(MessageType::PeerDelayRequest),
      0x3 => Ok(MessageType::PeerDelayResponse),
      0x8 => Ok(MessageType::FollowUp),
      0xA => Ok(MessageType::PeerDelayResponseFollowUp),
      0xB => Ok(MessageType::Announce),
      0xC => Ok(MessageType::Signaling),
      other => Err(format!("Unknown message type: {other:X}"))
    };
  }
}

impl Debug for MessageType
{
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    return match *self
    {
      MessageType::Sync1Step => formatter.write_str("Sync1Step"),
      MessageType::Sync2Step => formatter.write_str("Sync2Step"),
      MessageType::PeerDelayRequest => formatter.write_str("PeerDelayRequest"),
      MessageType::PeerDelayResponse => formatter.write_str("PeerDelayResponse"),
      MessageType::FollowUp => formatter.write_str("FollowUp"),
      MessageType::PeerDelayResponseFollowUp => formatter.write_str("PeerDelayResponseFollowUp"),
      MessageType::Announce => formatter.write_str("Announce"),
      MessageType::Signaling => formatter.write_str("Signaling"),
    };
  }
}

// impl Display for MessageType
// {
//   fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
//   {
//     return formatter.write_str(format!(
//       "{:X}",
//       match self
//       {
//         MessageType::Sync => 0x0,
//         MessageType::PeerDelayRequest => 0x2,
//         MessageType::PeerDelayResponse => 0x3,
//         MessageType::FollowUp => 0x8,
//         MessageType::PeerDelayResponseFollowUp => 0xA,
//         MessageType::Announce => 0xB,
//         MessageType::Signaling => 0xC,
//     }
//     ).as_str());
//   }
// }
