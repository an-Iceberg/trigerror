use std::{collections::VecDeque, fs::File, io::Write, path::PathBuf, process::exit, time::Duration};
use clap::Parser;
use pcap_file::pcap::{PcapPacket, PcapWriter};
use trigerror::{Protocol, cli::CLI, config::Config, create_capture_device, get_timestamp, protocol::GPTP, to_pcap, λ};

fn main()
{
  // let vec: Vec<u32> = vec![1,2,3,4,5];
  // let result = vec.iter()
  //   .map(λ!{n => n.pow(2)})
  //   .sum::<u32>();
  // dbg!{result};
  // exit(-1);

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

  // MONITORING PHASE

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
      let packet = to_pcap(capture_device.next_packet().unwrap());

      match protocol.validate_packet(&packet)
      {
        Ok(_) =>
        {
          let now = packet.timestamp;

          buffer.push_back(packet);
          // TODO: pop_front packets that exceed the time limit and buffer count.

          // FIX: this probably doesn't work correctly.
          // Discard packets that are too old.
          while now.abs_diff(buffer.front().unwrap().timestamp).as_millis() > config.time_before as u128
          { buffer.pop_front().unwrap(); }

          // Discard packets that make the buffer too big.
          while buffer.len() as u32 > config.count_before
          { buffer.pop_front().unwrap(); }
        }
        Err(error) =>
        {
          // WRITING PHASE

          let interface = config.interface.to_owned();
          let timestamp = get_timestamp();

          // Creating the capture file.
          let mut out_path = config.out_dir.clone();
          out_path.push(format!("trigerror_{interface}_{timestamp}.pcap"));
          let capture_file = File::create(out_path).expect("couldn't create packet capture file");
          let mut capture_writer = PcapWriter::new(capture_file).expect("Error writing to capture file");
          // Creating a file with information about the errors.
          let mut out_path = config.out_dir.clone();
          out_path.push(format!("trigerror_errors_{interface}_{timestamp}.txt"));
          let mut info_file = File::create(out_path).expect("couldn't create errors file");

          // FIX: This is very slow.
          for packet in buffer.iter()
          { capture_writer.write_packet(packet).expect("Error writing to capture file"); }

          let mut packet_number = buffer.len();
          let mut packet_counter = 0;
          let mut error_time = packet.timestamp;
          #[allow(non_snake_case)]
          let mut Δ_time;
          let mut retrigger_counter = 1;

          info_file.write_all(format!("#{}: {error}\n", packet_number).as_bytes()).expect("Error writing to errors file");

          loop
          {
            let packet = to_pcap(capture_device.next_packet().unwrap());
            packet_number += 1;
            capture_writer.write_packet(&packet).unwrap();
            if let Err(error) = protocol.validate_packet(&packet)
            {
              info_file.write_all(format!("#{packet_number}: {error}\n").as_bytes()).unwrap();

              if retrigger_counter < config.max_retriggers
              {
                retrigger_counter += 1;
                packet_counter = 0;
                Δ_time = Duration::from_millis(0);
              }
            }

            packet_counter += 1;
            Δ_time = packet.timestamp.abs_diff(error_time);

            if packet_counter > config.count_after || Δ_time.as_millis() > config.time_after as u128
            { break; }
          }

          buffer.clear();
        }
      }
    }
  }
}
