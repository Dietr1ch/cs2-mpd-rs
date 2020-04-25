{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    carnix
  ];

  RUST_BACKTRACE = 0;
}
