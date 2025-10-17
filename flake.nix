{
  description = "A simple flake for a Rust project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, devenv, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = devenv.lib.mkShell {
          modules = [
            ({ pkgs, ... }: {
              # This is the main module where you can configure your shell
              packages = [ pkgs.qemu-system-x86 ];
              languages.rust.enable = true;
            })
          ];
        };
      }
    );
}