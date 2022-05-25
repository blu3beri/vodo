all: lint build

lint:
	cargo clippy

run:
	cargo run

build:
	cargo build --release
