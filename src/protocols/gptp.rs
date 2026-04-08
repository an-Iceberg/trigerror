pub mod message_type;
pub mod flags;
pub mod header;
pub mod message;
pub mod md_sync_receive_sm;

use std::time::Duration;

use pcap_file::pcap::PcapPacket;
use crate::{bytes_to_u16, protocols::{Protocol, gptp::{md_sync_receive_sm::MDSyncReceiveStateMachine, message::GPTPMesage, message_type::MessageType}}};

pub struct GPTP
{
  // TODO: state machines for message types with persistent states.
  count: u32,
  md_sync_receive_state_machine: MDSyncReceiveStateMachine
}

impl GPTP
{
  pub fn new() -> Self
  {
    return GPTP
    {
      count: 0,
      md_sync_receive_state_machine: MDSyncReceiveStateMachine::new(Duration::from_millis(125))
    };
  }
}

impl Default for GPTP
{
  fn default() -> Self
  {
    return GPTP
    {
      count: u32::default(),
      md_sync_receive_state_machine: MDSyncReceiveStateMachine::new(Duration::default()),
    };
  }
}

impl Protocol for GPTP
{
  fn validate_packet(&mut self, packet: &PcapPacket) -> Result<(), String>
  {
    // self.count += 1;
    // self.count %= 200;

    // if self.count == 100 { return Err(format!("Protocol counted {} packets.", self.count)); }

    // PTP = 0x88f7

    // TODO: reverse byte order.

    // println!("ethertype=0x{:X}{:X}", packet.data[12], packet.data[13]);
    // dbg!{bytes_to_u16(packet.data[12], packet.data[13])};
    // println!("{:X}", bytes_to_u16(packet.data[12], packet.data[13]));

    // Not PTP; we don't care.
    if bytes_to_u16(packet.data[12], packet.data[13]) != 0x88f7
    { return Ok(()); }

    let payload = &packet.data[14..];

    // payload.iter()
    //   .for_each(|byte| print!("{byte:02x} "));

    // NOTE: do we need to reverse the bits with .reverse_bits()?
    // FIX: don't right shift!
    // FIX: apply mask to set upper 4 bits to 0s.
    // Validate message type.
    let message_type = MessageType::from_u8(payload[0] & 0b0000_1111)?;
    // FIX: some messages don't seem to be decoded correctly. Array out of bounds error.
    // dbg!{&message_type};

    // TODO: make this work
    let message = GPTPMesage::new(message_type, payload)?;
    // dbg!{&message};

    match message_type
    {
      MessageType::Sync | MessageType::FollowUp =>
        self.md_sync_receive_state_machine.change_state(packet.timestamp, message)?,
      _ => ()
    }

    // println!();
    // println!();

    return Ok(());
  }
}
