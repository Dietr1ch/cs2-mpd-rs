
build:
	cargo build

build_release:
	cargo build --release

run:
	cargo run

install: build_release
	cp -f ./target/release/csgo-mpd-rs ~/.bin/
