pub enum MessageType
{
  Sync =               0x0,
  PdelayReq =          0x2,
  PdelayResp =         0x3,
  FollowUp =           0x8,
  PdelayRespFollowUp = 0xA,
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
      0x2 => Ok(MessageType::PdelayReq),
      0x3 => Ok(MessageType::PdelayResp),
      0x8 => Ok(MessageType::FollowUp),
      0xA => Ok(MessageType::PdelayRespFollowUp),
      0xB => Ok(MessageType::Announce),
      0xC => Ok(MessageType::Signaling),
      other => Err(format!("Unknown message type: {other:X}"))
    };
  }
}
