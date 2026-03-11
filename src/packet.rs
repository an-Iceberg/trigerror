use pcap::{PacketCodec, PacketHeader};

/// Represents an ethernet frame with header and data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet
{
  pub header: PacketHeader,
  pub data: Box<[u8]>
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
