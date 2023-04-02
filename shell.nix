{ pkgs ? import <nixpkgs> { }, ... }:
with pkgs;
let
  unstable = import <nixos-unstable> {};
in
mkShell rec {
  buildInputs = with pkgs; [
    rustup
    gcc
    pkg-config
    lldb
    cargo
    rustc
    clippy
    openssl
    freetype
    expat
    unstable.just
    llvmPackages.bintools
    git-cliff
    git-lfs
    rust-analyzer
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  shellHook = ''
    export PATH="$CARGO_HOME:$PATH";
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath buildInputs}";
  '';
}
