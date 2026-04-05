use std::fmt::Debug;

pub enum MessageType
{
  Sync =               0x0,
  PdelayRequest =          0x2,
  PdelayResponse =         0x3,
  FollowUp =           0x8,
  PdelayResponseFollowUp = 0xA,
  Announce =           0xB,
  Signaling =          0xC,
}

impl MessageType
{
  pub fn from_u8(byte: u8) -> Result<MessageType, String>
  {
    return match byte
    {
      0x0 => Ok(MessageType::Sync),
      0x2 => Ok(MessageType::PdelayRequest),
      0x3 => Ok(MessageType::PdelayResponse),
      0x8 => Ok(MessageType::FollowUp),
      0xA => Ok(MessageType::PdelayResponseFollowUp),
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
      MessageType::Sync => formatter.write_str("Sync"),
      MessageType::PdelayRequest => formatter.write_str("PdelayRequest"),
      MessageType::PdelayResponse => formatter.write_str("PdelayResponse"),
      MessageType::FollowUp => formatter.write_str("FollowUp"),
      MessageType::PdelayResponseFollowUp => formatter.write_str("PdelayResponseFollowUp"),
      MessageType::Announce => formatter.write_str("Announce"),
      MessageType::Signaling => formatter.write_str("Signaling"),
    };
  }
}
