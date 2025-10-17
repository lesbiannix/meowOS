{
  description = "A simple flake for a Rust project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, devenv, rust-overlay }:
    let
      system = "x86_64-linux";
    in
    {
      devShells.${system}.default = devenv.lib.mkShell {
        modules = [
          ({ pkgs, ... }: {
            # This is the main module where you can configure your shell
            packages = [ pkgs.qemu-system-x86 ];
            languages.rust.enable = true;
          })
        ];
      };
    };
}