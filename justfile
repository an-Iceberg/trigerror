[default]
run:
  cargo run --color always --message-format human

build:
  cargo build --color always --message-format human

run-release:
  cargo run --release --color always --message-format human

build-release:
  cargo build --release --color always --message-format human

test:
  cargo test --color always --message-format human

clean:
  cargo clean --color always

# dev: build
#   ./target/debug/trigerror --help

give-net-cap:
  sudo setcap cap_net_raw+ep target/debug/trigerror

dev: build give-net-cap
  target/debug/trigerror

dev2: build
  target/debug/trigerror --interfaces "enp0s13f0u3u2i5" --protocols "ptp" --filters "ptp,gptp, http, ssh" --count-before 20 --retrigger false

dev3: build
  target/debug/trigerror --filters ""
