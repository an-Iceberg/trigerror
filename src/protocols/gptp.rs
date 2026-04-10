pub mod message_type;
pub mod flags;
pub mod header;
pub mod message;
pub mod sync_state_machine;
pub mod announce_state_machine;

use pcap_file::pcap::PcapPacket;
use crate::{protocols::{Protocol, gptp::{announce_state_machine::AnnounceStateMachine, message::GPTPMesage, message_type::MessageType, sync_state_machine::SyncStateMachine}}, utils::{bytes_to_u16, get_bit}};

#[derive(Default)]
pub struct GPTP
{
  // TODO: state machines for message types with persistent states.
  sync_sm: SyncStateMachine,
  announce_sm: AnnounceStateMachine,
}

impl GPTP
{
  pub fn new() -> Self
  {
    return GPTP
    {
      sync_sm: SyncStateMachine::new(),
      announce_sm: AnnounceStateMachine::new(),
    };
  }
}

impl Protocol for GPTP
{
  fn validate_packet(&mut self, packet: &PcapPacket) -> Result<(), String>
  {
    // TODO: collect all errors in Vec<String> and return that. (breaking API change)

    // PTP = 0x88f7

    // println!("ethertype=0x{:X}{:X}", packet.data[12], packet.data[13]);
    // dbg!{bytes_to_u16(packet.data[12], packet.data[13])};
    // println!("{:X}", bytes_to_u16(packet.data[12], packet.data[13]));

    // Not PTP; we don't care.
    if bytes_to_u16(packet.data[12], packet.data[13]) != 0x88f7
    { return Ok(()); }

    let ether_source = (
      packet.data[0],
      packet.data[1],
      packet.data[2],
      packet.data[3],
      packet.data[4],
      packet.data[5],
    );
    let ether_destination = (
      packet.data[6],
      packet.data[7],
      packet.data[8],
      packet.data[9],
      packet.data[10],
      packet.data[11],
    );

    let payload = &packet.data[14..];

    // payload.iter()
    //   .for_each(|byte| print!("{byte:02x} "));

    // println!("payload[0] = {:0X}, {:?}", payload[0] & 0b0000_1111, message_type);

    // TODO: verify, that this works.
    let message = GPTPMesage::new(
      MessageType::from_u8(payload[0] & 0b0000_1111, get_bit(payload[6], 1))?,
      payload
    )?;

    // TODO: domain nr. (probably with a HashMap (use hashbrown as needed)).
    // TODO: ethernet source address

    use MessageType::{Sync1Step, Sync2Step, FollowUp, Announce};

    match *message.get_type()
    {
      Sync1Step | Sync2Step | FollowUp =>
      {
        self.sync_sm.validate_state(*message.get_type())?;
        self.sync_sm.validate_timing(
          packet.timestamp,
          message.header().message_interval(),
          *message.get_type()
        )?;
      }

      Announce =>
      {
        self.announce_sm.validate_state(*message.get_type())?;
        self.announce_sm.validate_timing(
          packet.timestamp,
          message.header().message_interval(),
          *message.get_type()
        )?;
      }

      _ => ()
    }

    return Ok(());
  }
}
