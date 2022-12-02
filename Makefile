run:
	RUST_LOG=debug cargo run -- $(ARGS)

build:
	cargo build --package bin-rp2040 --target thumbv6m-none-eabi --release

test:
	cargo test -p libpoly