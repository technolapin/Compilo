all: build

build:
	cargo build --release
check:
	cargo test -- --show-output

target/release/compilo: build

run: target/release/compilo
	$(foreach file, $(wildcard scripts/*.tig), ./target/release/compilo $(file);)

