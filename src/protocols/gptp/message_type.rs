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
