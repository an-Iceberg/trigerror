pub mod message_type;
pub mod flags;
pub mod header;
pub mod message;
pub mod md_sync_receive_sm;

use libc::PACKET_KERNEL;
use pcap_file::pcap::PcapPacket;
use crate::{Protocol, bytes_to_u16, get_ether_type};

type Octet = u8;

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
    self.count %= 2_000;

    if self.count == 1_000 { return Err(format!("Protocol counted {} packets.", self.count)); }

    // Not PTP, we don't care.
    if get_ether_type(bytes_to_u16(packet.data[12], packet.data[13])) != "PTP"
    { return Ok(()); }

    return Ok(());
  }
}
