pub struct EthFrame
{
  pub(crate) destination: (u8, u8, u8, u8, u8, u8),
  pub(crate) source: (u8, u8, u8, u8, u8, u8),
  pub(crate) ether_type: u16,
  pub(crate) payload: Vec<u8>,
  // size: u16,
}

impl EthFrame {}

pub enum EtherType
{
  IPv4 = 0x0800,
  Chaosnet = 0x0804,
  ARP = 0x0806,
  WakeOnLAN = 0x0842,
  StreamReservationProtocol = 0x22ea,
  AVTP = 0x22f0,
  IETFTRILLProtocol = 0x22f3,
  DECMOPRC = 0x6002,
  // etc. etc. we don't need this rn
}
