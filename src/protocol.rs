use pcap_file::pcap::PcapPacket;

use crate::get_timestamp;

#[derive(Default)]
pub(crate) struct Protocol
{
  count: u32
}

impl Protocol
{
  pub fn take_packet(&mut self, packet: PcapPacket<'static>) -> Result<PcapPacket<'static>, (PcapPacket<'static>, String)>
  {
    self.count += 1;

    if self.count > 50 { return Err((packet, get_timestamp()));   }
    else { return Ok(packet); }
  }
}

// pub(crate) trait ProtocolTrait
// {}
