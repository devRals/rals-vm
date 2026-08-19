{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk/master";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    naersk,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };

        naersk-lib = pkgs.callPackage naersk {};

        rals-vm = naersk-lib.buildPackage {
          src = ./.;
        };

        rvm = {
          type = "app";
          program = "${rals-vm}/bin/rvm";
        };
        rasm = {
          type = "app";
          program = "${rals-vm}/bin/rasm";
        };
      in {
        packages = {
          default = rals-vm;
        };

        apps = {
          inherit rvm rasm;
          default = rvm;
        };

        devShells.default = with pkgs;
          mkShell {
            packages = [
              mdbook
              cargo
              rustc
              rustfmt
              clippy
              rust-analyzer
            ];

            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      }
    );
}
