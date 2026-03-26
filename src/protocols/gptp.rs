use pcap_file::pcap::PcapPacket;
use crate::Protocol;

pub struct GPTP
{
  // TODO: state machines for message types with persistent states.
  count: u32,
}

impl GPTP
{
  pub fn new() -> Self { return GPTP { count: 0, }; }
}

impl Default for GPTP
{
  fn default() -> Self { return GPTP { count: u32::default() }; }
}

impl Protocol for GPTP
{
  fn validate_packet(&mut self, packet: &PcapPacket) -> Result<(), String>
  {
    // Sync timeout, frame comes periodically, record when packet is missing (datafield last_sync_timer)
    // Figure 11-6
    self.count += 1;
    self.count %= 1000;

    if self.count == 500 { return Err("Protocol packet count reached 500".to_string()); }

    return Ok(());
  }
}
