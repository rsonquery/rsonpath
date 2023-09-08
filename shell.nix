{ nixpkgs ? import <nixpkgs> { }, ... }:
let
  stable = import (nixpkgs.fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs";
    rev = "4ecab3273592f27479a583fb6d975d4aba3486fe";
    sha256 = "btHN1czJ6rzteeCuE/PNrdssqYD2nIA4w48miQAFloM=";
  }) {};
  unstable = import (nixpkgs.fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs";
    rev = "7403dd3fe24c2133089bdeaef593ed02c34fcdae";
    sha256 = "vfHkC88PjYIpI1rsAOa7Cr6fXvPXxKM3tDvMPlgv0NM=";
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
      rustfmt
    ]) ++ (with unstable; [
      just
    ]);

  RUST_SRC_PATH = "${nixpkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  shellHook = ''
    export RUSTUP_TOOLCHAIN="nightly";
    export PATH="$CARGO_HOME:$PATH";
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${nixpkgs.lib.makeLibraryPath buildInputs}";
  '';
}
