{
  rustPlatform,
  lib,
  darwin,
  stdenv,
}:
rustPlatform.buildRustPackage {
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
  pname = "mole";
  version = "0.0.1";
  buildInputs = lib.optional stdenv.isDarwin darwin.apple_sdk.frameworks.IOKit;
}
