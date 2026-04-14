# `trigerror`

Utility for recording network traffic upon encountering an error in a packet.

- TODO: maybe it would be a good idea to create a `struct PacketState` where all possible errors are set as one big bitfield?
- TODO: logo
- TODO: calculate filesize from captured packets
- DONE: for writing `.pcapng` files, 2 crates are of interest: `pcapng-writer` and `pcap-file`
- NOTE: https://biot.com/capstats/bpf.html
- NOTE: [intona ethernet tap source code](https://github.com/intona/ethernet-debugger#readme)
- NOTE: https://intona.eu/en/doc/ethernet-debugger/#IN3032UG:EthernetDebuggerUserGuide-Linux,macOS
- SOLL-ZIEL: store data as `.pcapng` files
- NOTE: linkspeed
- TODO: calculate size of buffer (important if operating on RPi with max RAM of 8GB) (constrained memory environment)
- NOTE: param prio: 1. time 2. memory size 3. packet count. if no more memory, trim start of error
- TODO: consider some logging library: https://docs.rs/log/latest/log/
- TODO: in the BA paper link to dependencies directly
- TODO: abstract auch noch mehrwert und was dabei rauskam (wichtigsten ergebnisse)
- TODO: make use of [`PcapParser`](https://docs.rs/pcap-file/latest/pcap_file/pcap/struct.PcapParser.html)

## Useful resources
- [an alternative way to handle `enum`s using generics](https://stackoverflow.com/questions/72438594/how-can-i-use-enum-variants-as-generic-type#answer-72438660)
- [Rust parallelism](https://www.youtube.com/watch?v=AiSl4vf40WU)
- [gPTP from ZHAW](https://www.zhaw.ch/en/engineering/institutes-centres/ines/communication-network-engineering/high-precision-time-synchronization-with-ptp-and-gptp-new-page)
- [PTP](https://en.wikipedia.org/wiki/Precision_Time_Protocol)
- [Rust crate implementing PTP](https://crates.io/crates/statime)
- [config directory in OS](https://docs.rs/dirs/latest/dirs/fn.config_dir.html)
- [Rust `pcap` crate](https://docs.rs/rpcap/latest/rpcap/)
- [Rust `cli` help](https://rust-cli.github.io/book/index.html)
- https://stackoverflow.com/questions/28823788/how-do-i-clear-the-current-line-of-stdout
- https://stackoverflow.com/questions/2388090/how-to-delete-and-replace-last-line-in-the-terminal-using-bash
- https://stackoverflow.com/questions/1953300/how-to-send-pcap-file-packets-on-nic
- [EtherType](https://en.wikipedia.org/wiki/EtherType#Values)
- [write to file](https://www.reddit.com/r/learnrust/comments/ggge3j/what_is_the_proper_way_in_rust_of_writing_into_a/)
- [pcap file format](https://www.endace.com/learn/what-is-a-pcap-file)
- [UNIX epoch converter](https://www.epochconverter.com/)
- [ZHAW Overleaf template](https://www.overleaf.com/latex/templates/zhaw-thesis-template-v2-dot-0/dgmxrbjjwsgy)
- state machine libraries
  - https://github.com/titanclass/edfsm
  - https://github.com/eugene-babichenko/rust-fsm
  - https://github.com/mdeloof/statig
  - https://crates.io/search?q=finite%20state%20machine
- [state machine pattern for Rust](https://refactoring.guru/design-patterns/state/rust/example)

## References
- [`timeval` `struct` fields meaning](https://man7.org/linux/man-pages/man3/timeval.3type.html)
