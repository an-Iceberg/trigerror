use pcap::{PacketCodec, PacketHeader};
use crate::{bytes_to_u16, eth_frame::EthFrame};

// TODO: extract an ethernet frame from this.
/// Represents a captured packet.
#[derive(Debug, Clone)]
pub struct Packet
{
  pub header: PacketHeader,
  pub data: Vec<u8>, // Formerly Box<[u8]>
}

pub struct Codec;

impl PacketCodec for Codec
{
  type Item = Packet;

  fn decode(&mut self, packet: pcap::Packet<'_>) -> Self::Item
  {
    return Packet
    {
      header: *packet.header,
      data: packet.data.into(),
    };
  }
}

impl Packet
{
  pub fn to_eth_frame(&mut self) -> EthFrame
  {
    return EthFrame
    {
      destination: (self.data[0], self.data[1], self.data[2], self.data[3], self.data[4], self.data[5]),
      source: (self.data[6], self.data[7], self.data[8], self.data[9], self.data[10], self.data[11]),
      ether_type: bytes_to_u16(self.data[12], self.data[13]),
      payload: self.data[14..].into(),
    };
  }
}
