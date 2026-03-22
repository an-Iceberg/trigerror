use pcap_file::pcap::PcapPacket;
use crate::{Protocol, get_timestamp};

pub struct GPTP
{
  count: u32
}

impl GPTP
{
  pub fn new() -> Self { return GPTP::default(); }
}

impl Default for GPTP
{
  fn default() -> Self { return GPTP { count: u32::default() }; }
}

impl Protocol for GPTP
{
  fn validate_packet(&mut self, packet: &PcapPacket) -> Result<(), String>
  {
    self.count += 1;
    self.count %= 100;

    if self.count >= 30 { return Err("Packet count reached 20".to_string()); }

    return Ok(());
  }
}
