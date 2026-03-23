use pcap_file::pcap::PcapPacket;

pub trait Protocol
{
  fn validate_packet(&mut self, packet: &PcapPacket) -> Result<(), String>;
}
