pub mod message_type;
pub mod flags;
pub mod header;
pub mod message;
pub mod md_sync_receive_sm;

use pcap_file::pcap::PcapPacket;
use crate::{bytes_to_u16, protocols::{Protocol, gptp::{message::GPTPMesage, message_type::MessageType}}};

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

    // VLAN = 0x8100
    // PTP = 0x88f7

    // Determine EtherType.
    let mut ether_type = bytes_to_u16(packet.data[12], packet.data[13]);
    let mut vlan = false;

    // Handle VLAN.
    if ether_type == 0x8100
    {
      // Either 15, 16 or 16, 17
      ether_type = bytes_to_u16(packet.data[15], packet.data[16]);
      vlan = true;
    }

    // FIX: VLAN doesn't have PTP
    // Not PTP; we don't care.
    if ether_type != 0x88f7 { return Ok(()); }

    // We now only have PTP packets.

    // Extracting the payload from the ethernet frame.
    let payload: &[u8] = match vlan
    {
      false => packet.data[14..].into(),
      true => packet.data[17..].into()
    };

    // NOTE: do we need to reverse the bits with .reverse_bits()?
    // FIX: don't right shift!
    // FIX: apply mask to set upper 4 bits to 0s.
    // Validate message type.
    let message_type = MessageType::from_u8(payload[0] >> 4)?;

    let message = GPTPMesage::new(message_type, payload);

    return Ok(());
  }
}
