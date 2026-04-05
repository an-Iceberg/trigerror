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
    self.count %= 200;

    if self.count == 100 { return Err(format!("Protocol counted {} packets.", self.count)); }

    // PTP = 0x88f7

    // TODO: reverse byte order.

    // println!("ethertype=0x{:X}{:X}", packet.data[12], packet.data[13]);
    // dbg!{bytes_to_u16(packet.data[12], packet.data[13])};
    // println!("{:X}", bytes_to_u16(packet.data[12], packet.data[13]));

    // return Ok(());

    // Not PTP; we don't care.
    if bytes_to_u16(packet.data[12], packet.data[13]) != 0x88f7
    { return Ok(()); }

    let payload = &packet.data[14..];

    // NOTE: do we need to reverse the bits with .reverse_bits()?
    // FIX: don't right shift!
    // FIX: apply mask to set upper 4 bits to 0s.
    // Validate message type.
    let message_type = MessageType::from_u8(payload[0] & 0b0000_1111)?;
    dbg!{&message_type};

    // TODO: make this work
    let message = GPTPMesage::new(message_type, payload);

    return Ok(());
  }
}
