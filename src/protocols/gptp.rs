pub mod message_type;
pub mod flags;
pub mod header;
pub mod message;
pub mod md_sync_receive_sm;

use std::time::Duration;
use pcap_file::pcap::PcapPacket;
use crate::{bytes_to_u16, get_bit, protocols::{Protocol, gptp::{md_sync_receive_sm::MDSyncReceiveStateMachine, message::GPTPMesage, message_type::MessageType}}};

#[derive(Default)]
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
      md_sync_receive_state_machine: MDSyncReceiveStateMachine::new()
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

    // TODO: verify, that this works.
    // Validate message type.
    let message_type = MessageType::from_u8(
      payload[0] & 0b0000_1111,
      get_bit(payload[6], 1)
    )?;
    // println!("payload[0] = {:0X}, {:?}", payload[0] & 0b0000_1111, message_type);

    let message = GPTPMesage::new(message_type, payload)?;

    // TODO: domain nr. (probably with a HashMap (use hashbrown as needed)).
    // TODO: ethernet source address

    message.header();

    // match message_type
    // {
    //   MessageType::Sync1Step | MessageType::Sync2Step | MessageType::FollowUp =>
    //     self.md_sync_receive_state_machine.validate(packet.timestamp, message)?,
    //   _ => ()
    // }

    // match message
    // {
    //   GPTPMesage::Announce { header, current_utc_offset, grandmaster_priority_1, grandmaster_clock_quality, grandmaster_priority_2, grandmaster_identity, steps_removed, time_source, tlv_type, length_field, path_sequence } => todo!(),
    //   GPTPMesage::Signaling { header, target_port_identity } => todo!(),
    //   GPTPMesage::Sync1Step { header, origin_timestamp, tlv_type, length_field, organization_id, organization_sub_type, cumulative_scaled_rate_offset, gm_time_base_indicator, last_gm_phase_change, scaled_last_gm_frequency_change } => todo!(),
    //   GPTPMesage::Sync2Step { header, reserved } => todo!(),
    //   GPTPMesage::FollowUp { header } => todo!(),
    //   GPTPMesage::PeerDelayRequest { header } => todo!(),
    //   GPTPMesage::PeerDelayResponse { header, request_receipt_timestamp, requesting_port_identity } => todo!(),
    //   GPTPMesage::PeerDelayResponseFollowUp { header, response_origin_timestamp, requesting_port_identity } => todo!(),
    // }

    return Ok(());
  }
}
