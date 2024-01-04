{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    pkg-config
    (dbus // { meta.outputsToInstall = [ "dev" ]; })
  ];

  RUST_BACKTRACE = 1;
}
