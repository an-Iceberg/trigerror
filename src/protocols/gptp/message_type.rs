use std::fmt::Display;

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

impl Display for MessageType
{
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    return formatter.write_str(match self
    {
      MessageType::Sync1Step => "Sync1Step",
      MessageType::Sync2Step => "Sync2Step",
      MessageType::PeerDelayRequest => "PeerDelayRequest",
      MessageType::PeerDelayResponse => "PeerDelayResponse",
      MessageType::FollowUp => "FollowUp",
      MessageType::PeerDelayResponseFollowUp => "PeerDelayResponseFollowUp",
      MessageType::Announce => "Announce",
      MessageType::Signaling => "Signaling",
    });
  }
}
