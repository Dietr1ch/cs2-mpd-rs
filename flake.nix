{
  description = "A silly CS2 MPD integration demo";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forEachSupportedSystem =
        f:
        nixpkgs.lib.genAttrs supportedSystems (
          system:
          f {
            pkgs = import nixpkgs {
              inherit system;
              overlays = [
                rust-overlay.overlays.default
                self.overlays.default
              ];
            };
          }
        );
    in
    {
      nix.nixPath = [
        "nixpkgs=${nixpkgs}"
      ];

      overlays.default = final: prev: {
        rustToolchain = prev.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      };

      devShells = forEachSupportedSystem (
        { pkgs }:
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              openssl

              rustToolchain

              llvmPackages.bintools
              llvmPackages.bolt
              rustc
              cargo
              rustup

              rust-jemalloc-sys
            ];

            packages = with pkgs; [
              # Git
              ripsecrets

              # Nix
              nixd
              nixfmt-rfc-style

              # Rust
              cargo-audit
              cargo-bloat
              cargo-criterion
              cargo-deny
              cargo-edit
              cargo-expand
              cargo-modules
              cargo-nextest
              cargo-outdated
              cargo-public-api
              cargo-semver-checks
              cargo-toml-lint
              cargo-udeps

              bacon

              just
            ];

            env = {
              # Rust
              ## Required by rust-analyzer
              RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
            };
          };
        }
      ); # ..devShells
    };
}
