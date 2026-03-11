build:
  cargo build --color always --message-format human
  sudo setcap cap_net_raw+ep target/debug/trigerror

run: build
  target/debug/trigerror

build-release:
  cargo build --release --color always --message-format human
  sudo setcap cap_net_raw+ep target/release/trigerror

run-release: build-release
  target/release/trigerror

test:
  cargo test --color always --message-format human

clean:
  cargo clean --color always

# dev: build
#   ./target/debug/trigerror --help

# give-net-cap:
#   sudo setcap cap_net_raw+ep target/debug/trigerror

dev: build
  target/debug/trigerror

dev2: build
  target/debug/trigerror --interfaces "enp0s13f0u3u2i5" --protocols "ptp" --filters "ptp,gptp, http, ssh" --count-before 20 --retrigger false

dev3: build
  target/debug/trigerror --filters ""

dev4: build
  target/debug/trigerror --interfaces "wlan0"
