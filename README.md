# `trigerror`

- TODO: calculate filesize from captured packets
- TODO: sync, announce and Pdelay messages are important.
- TODO: https://wiki.archlinux.org/title/Capabilities
- TODO: `.pcap` vs. `.pcapng`
- TODO: research: `.pcap`/`.pcapng` file format
- TODO: use/extend `VecDeque` as a ring buffer.
- TODO: for writing `.pcapng` files, 2 crates are of interest: `pcapng-writer` and `pcap-file`
- NOTE: https://biot.com/capstats/bpf.html
- NOTE: [intona ethernet tap source code](https://github.com/intona/ethernet-debugger#readme)
- NOTE: https://intona.eu/en/doc/ethernet-debugger/#IN3032UG:EthernetDebuggerUserGuide-Linux,macOS
- SOLL-ZIEL: store data as `.pcapng` files
- NOTE: linkspeed
- TODO: calculate size of buffer (important if operating on RPi with max RAM of 8GB) (constrained memory environment)
- TODO: for testing: 100th packet as error trigger
- NOTE: param prio: 1. time 2. memory size 3. packet count\
  if no more memory, trim start of error

## Useful resources
- https://www.zhaw.ch/en/engineering/institutes-centres/ines/communication-network-engineering/high-precision-time-synchronization-with-ptp-and-gptp-new-page
- https://en.wikipedia.org/wiki/Precision_Time_Protocol
- https://crates.io/crates/statime
- https://docs.rs/dirs/latest/dirs/fn.config_dir.html
- https://docs.rs/rpcap/latest/rpcap/
- https://rust-cli.github.io/book/index.html
- https://stackoverflow.com/questions/28823788/how-do-i-clear-the-current-line-of-stdout
- https://stackoverflow.com/questions/2388090/how-to-delete-and-replace-last-line-in-the-terminal-using-bash
