use pcap::{PacketCodec, PacketHeader};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedPacket
{
  pub header: PacketHeader,
  pub data: Box<[u8]>
}

pub struct Codec;

impl PacketCodec for Codec
{
  type Item = OwnedPacket;

  fn decode(&mut self, packet: pcap::Packet<'_>) -> Self::Item
  {
    return OwnedPacket
    {
      header: *packet.header,
      data: packet.data.into(),
    };
  }
}
