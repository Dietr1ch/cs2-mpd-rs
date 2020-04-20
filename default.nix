with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "csgo-mpd-rs";
  buildInputs = [
    rustChannels.nightly.cargo
    rustChannels.nightly.rust
  ];
}

