all: build

build:
	cargo build
check:
	cargo test -- --show-output
