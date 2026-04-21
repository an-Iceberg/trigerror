build-dev:
  mkdir -p bin
  cargo build --color always --message-format human
  cp target/debug/trigerror bin
  mv bin/trigerror bin/trigerror_gptp_dev
  sudo setcap cap_net_raw+ep bin/trigerror_gptp_dev

run-dev: build-dev
  bin/trigerror_gptp_dbg

build:
  mkdir -p bin
  cargo build --release --color always --message-format human
  cp target/release/trigerror bin
  mv bin/trigerror bin/trigerror_gptp
  sudo setcap cap_net_raw+ep bin/trigerror_gptp

run: build
  bin/trigerror_gptp

test:
  cargo test --color always --message-format human

clean:
  cargo clean --color always
  rm -r bin/*

dev: build-dev
  bin/trigerror_gptp_dev --help

dev-school: build-dev
  bin/trigerror_gptp_dev --interfaces "wlan0"

dev-home: build-dev
  bin/trigerror_gptp_dev --interfaces "enp0s13f0u3u2i5"

dev-home-2: build-dev
  bin/trigerror_gptp_dev --create-default-config

install: build
  sudo mv bin/trigerror_gptp /usr/bin
  sudo setcap cap_net_raw+ep /usr/bin/trigerror_gptp
