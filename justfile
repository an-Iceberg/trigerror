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

dev: build
  ./target/debug/trigerror --help
