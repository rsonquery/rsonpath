{ nixpkgs ? import <nixpkgs> { }, ... }:
let
  stable = import (nixpkgs.fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs";
    rev = "884e3b68be02ff9d61a042bc9bd9dd2a358f95da";
    sha256 = "ISWz16oGxBhF7wqAxefMPwFag6SlsA9up8muV79V9ck=";
  }) {};
  unstable = import (nixpkgs.fetchFromGitHub {
    owner = "NixOS";
    repo = "nixpkgs";
    rev = "53dad94e874c9586e71decf82d972dfb640ef044";
    sha256 = "9FNIqrxDZgSliGGN2XJJSvcDYmQbgOANaZA4UWnTdg4=";
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
    ]) ++ (with unstable; [
      just
    ]);

  RUST_SRC_PATH = "${nixpkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  shellHook = ''
    export PATH="$CARGO_HOME:$PATH";
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${nixpkgs.lib.makeLibraryPath buildInputs}";
  '';
}
