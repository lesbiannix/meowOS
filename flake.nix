{
  description = "A simple flake for a Rust project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    devenv.url = "github:cachix/devenv";
  };

  outputs = { self, nixpkgs, rust-overlay, devenv }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
        overlays = [ rust-overlay.overlays.default ];
      };
    in
    {
      devShells.x86_64-linux.default = devenv.lib.mkShell {
        inherit pkgs;
        packages = [
          pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml
          pkgs.qemu-system-x86
        ];
      };
    };
}