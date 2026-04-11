build:
  mkdir -p bin
  cargo build --color always --message-format human
  cp target/debug/trigerror bin
  mv bin/trigerror bin/trigerror_gptp_dbg
  sudo setcap cap_net_raw+ep bin/trigerror_gptp_dbg

run: build
  bin/trigerror_gptp_dbg

build-release:
  mkdir -p bin
  cargo build --release --color always --message-format human
  cp target/release/trigerror bin
  mv bin/trigerror bin/trigerror_gptp
  sudo setcap cap_net_raw+ep bin/trigerror_gptp

run-release: build-release
  bin/trigerror_gptp

test:
  cargo test --color always --message-format human

clean:
  cargo clean --color always
  rm -r bin/*

dev: build
  bin/trigerror_gptp_dbg --help

dev-school: build
  bin/trigerror_gptp_dbg --interfaces "wlan0"

dev-home: build
  bin/trigerror_gptp_dbg --interfaces "enp0s13f0u3u2i5"

dev-home-2: build
  bin/trigerror_gptp_dbg --create-default-config

install: build-release
  sudo mv bin/trigerror_gptp /usr/bin
  sudo setcap cap_net_raw+ep /usr/bin/trigerror_gptp
