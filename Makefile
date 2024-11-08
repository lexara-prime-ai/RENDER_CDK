format:
	cargo fmt --quiet

lint:
	cargo clippy

release:
	cargo build --release

debug:
	cargo build

run:
	cargo run