use pcap_file::pcap::PcapPacket;

pub mod gptp;

pub trait Protocol
{
  fn validate_packet(&mut self, packet: &PcapPacket) -> Result<(), Vec<String>>;
}
