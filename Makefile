
build:
	nix-shell --pure --command 'cargo build'

release:
	nix-shell --pure --command 'cargo build --release'

run:
	nix-shell --pure --command 'cargo run'

install: release
	cp -f ./target/release/csgo-mpd-rs ~/.bin/
