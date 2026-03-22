use std::fs::File;

use pcap_file::pcap::{PcapPacket, PcapWriter};

pub struct Writer
{
  // capture_file: File,
  pcap_writer: PcapWriter<File>,
  reasons_file: File
}

impl Writer
{
  pub fn new(interface: String, timestamp: String) -> Writer
  {
    // let capture_file = File::create(format!("trigerror_{interface}_{timestamp}.pcap")).expect("couldn't create file");
    // let pcap_writer =  PcapWriter::new(File::create(format!("trigerror_{interface}_{timestamp}.pcap")).expect("couldn't create file")).expect("Error writing file");
    // let reasons_file = File::create(format!("trigerror_info_{interface}_{timestamp}.txt")).expect("couldn't create reasons file");
    return Writer
    {
      // capture_file,
      pcap_writer: PcapWriter::new(File::create(format!("trigerror_{interface}_{timestamp}.pcap")).expect("couldn't create file")).expect("Error writing file"),
      reasons_file: File::create(format!("trigerror_info_{interface}_{timestamp}.txt")).expect("couldn't create reasons file")
    }
  }

  pub fn write_packet(&mut self, packet: &PcapPacket)
  { self.pcap_writer.write_packet(packet).unwrap(); }
}
