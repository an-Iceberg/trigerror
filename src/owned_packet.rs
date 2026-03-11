use pcap::{PacketCodec, PacketHeader};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketOwned
{
  pub header: PacketHeader,
  pub data: Box<[u8]>
}

pub struct Codec;

impl PacketCodec for Codec
{
  type Item = PacketOwned;

  fn decode(&mut self, packet: pcap::Packet<'_>) -> Self::Item
  {
    return PacketOwned
    {
      header: *packet.header,
      data: packet.data.into(),
    };
  }
}
