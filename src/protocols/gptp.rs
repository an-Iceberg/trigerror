pub mod message_type;
pub mod flags;
pub mod header;
pub mod message;
pub mod sync_state_machine;
pub mod announce_state_machine;
pub mod state_machines;

use pcap_file::pcap::PcapPacket;
use crate::{mac::MAC, protocols::{Protocol, gptp::{announce_state_machine::AnnounceStateMachine, message::GPTPMesage, message_type::MessageType, state_machines::{announce::AnnounceSM, peer_delay::PeerDelaySM, signaling::SignalingSM, sync::SyncSM}, sync_state_machine::SyncStateMachine}}, utils::{bytes_to_u16, get_bit}};

#[derive(Default)]
pub struct GPTP
{
  sync_sm: SyncSM,
  announce_sm: AnnounceSM,
  peer_delay_sm: PeerDelaySM,
  signaling_sm: SignalingSM,
}

impl GPTP
{
  pub fn new() -> Self
  {
    return GPTP
    {
      sync_sm: SyncSM::default(),
      announce_sm: AnnounceSM::default(),
      peer_delay_sm: PeerDelaySM::default(),
      signaling_sm: SignalingSM::default(),
    };
  }
}

impl Protocol for GPTP
{
  fn validate_packet(&mut self, packet: &PcapPacket) -> Result<(), Vec<String>>
  {
    // TODO: collect all errors in Vec<String> and return that. (breaking API change)

    // PTP = 0x88f7

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

    let payload = &packet.data[14..];

    let message_type = match MessageType::from_u8(payload[0] & 0b0000_1111, get_bit(payload[6], 1))
    {
      Ok(message_type) => message_type,
      Err(error) => return Err(vec![error]),
    };

    // println!(
    //   "packet No.: {}, type: {message_type:?}, is: {:0X}, mask supplied: {:0X}",
    //   self.counter,
    //   payload[0],
    //   payload[0] & 0b0000_1111
    // );
    // payload.iter()
    //   .for_each(|byte| print!("{byte:02x} "));
    // println!();

    // TODO: verify, that this works.
    let message = match GPTPMesage::new(message_type, payload)
    {
      Ok(message) => message,
      Err(error) => return Err(vec![error])
    };

    // TODO: domain nr. (probably with a HashMap (use hashbrown as needed)).

    use MessageType::{Sync1Step, Sync2Step, FollowUp, Announce};

    return match message.get_type()
    {
      Sync1Step | Sync2Step | FollowUp =>
        self.sync_sm.validate(
          message.get_type(),
          packet.timestamp,
          message.header().message_interval(),
          MAC::from_bytes(ether_source)
        ),
      Announce =>
        self.announce_sm.validate(
          message.get_type(),
          packet.timestamp,
          message.header().message_interval(),
          MAC::from_bytes(ether_source)
        ),
      _ => Ok(())
    };
  }
}
