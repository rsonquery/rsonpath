{ nixpkgs ? import <nixpkgs> { }, ... }:
let
  stable = import (nixpkgs.fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs";
    rev = "57a0c58e728a28bc21eed85055eac9724a08f69a";
    sha256 = "jZM9mIChS+tdrLt3/R6WnFNJCGcD8M/TpUgxrBgAN3M=";
  }) {};
in stable.mkShell rec {
  buildInputs =
    (with stable; [
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
      llvmPackages.bintools
      git-lfs
      git-cliff
      rust-analyzer
      cargo-hack
      cargo-watch
      just
      rustfmt
    ]);

  RUST_SRC_PATH = "${nixpkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  shellHook = ''
    export RUSTUP_TOOLCHAIN="nightly";
    export PATH="$CARGO_HOME:$PATH";
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${nixpkgs.lib.makeLibraryPath buildInputs}";
  '';
}
