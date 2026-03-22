use std::{collections::VecDeque, fs::File, io::Write, path::PathBuf};
use clap::Parser;
use pcap_file::pcap::{PcapPacket, PcapWriter};
use trigerror::{Protocol, cli::CLI, config::Config, create_capture_device, get_timestamp, pac2pac, protocol::GPTP};

fn main()
{
  let cli = CLI::parse();
  let mut interfaces = vec![];
  let mut config = Config::new();

  // CONFIGURE PHASE

  // Check if config file location has been specified.
  if let Some(config_file_path) = cli.config_file_location
  {
    // Read config from said location and determine interfaces.
    if let Some(ifaces) = config.set_from_ini(config_file_path)
    { interfaces = ifaces; }
  }
  else
  {
    // Read config file in cwd (if exists).
    if let Some(ifaces) = config.set_from_ini(PathBuf::from("trigerror.ini"))
    { interfaces = ifaces; }
    // Overwrite options from CLI.
    if let Some(ifaces) = config.set_from_cli(cli)
    { interfaces = ifaces; }
  }

  // CAPTURE PHASE

  // TODO: create a thread for each interface. HELP: https://doc.rust-lang.org/rust-by-example/std_misc/threads.html

  {
    let first_iface = interfaces.first().unwrap();
    config.interface = first_iface.to_owned();
    let mut capture_device = create_capture_device(&config);
    let mut buffer: VecDeque<PcapPacket> = VecDeque::new();
    let mut protocol = GPTP::new();
    // let mut recording = Recording::new(config);

    // TODO: this is formulated imperatively just so it works. Use better structures.
    loop
    {
      let packet = pac2pac(capture_device.next_packet().unwrap());

      match protocol.validate_packet(&packet)
      {
        Ok(_) => buffer.push_back(packet),
        Err(error) =>
        {
          let interface = config.interface.to_owned();
          let timestamp = get_timestamp();

          let mut out_path = config.out_dir.clone();
          out_path.push(format!("trigerror_{interface}_{timestamp}.pcap"));
          let capture_file = File::create(out_path).expect("couldn't create packet capture file");
          let mut capture_writer = PcapWriter::new(capture_file).expect("Error writing file");
          let mut out_path = config.out_dir.clone();
          out_path.push(format!("trigerror_info_{interface}_{timestamp}.txt"));
          let mut info_file = File::create(out_path).expect("couldn't create info file");

          let mut packet_number = buffer.len();
          let mut packet_counter = buffer.len();
          // FIX: This is very slow.
          for packet in buffer.iter()
          { capture_writer.write_packet(packet).unwrap(); }
          buffer.clear();

          packet_number += 1;

          info_file.write_all(format!("#{}: {error}\n", packet_number).as_bytes()).unwrap();

          loop
          {
            // TODO: escape this loop once the post_trigger conditions are met.
            packet_number += 1;

            let packet = pac2pac(capture_device.next_packet().unwrap());
            capture_writer.write_packet(&packet).unwrap();

            if let Err(error) = protocol.validate_packet(&packet)
            {
              info_file.write_all(format!("#{}: {error}\n", packet_number).as_bytes()).unwrap();
            }
          }
        }
      }
    }
  }
}
