{
  rustPlatform,
}:
rustPlatform.buildRustPackage {
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
  pname = "mole";
  version = "0.0.1";
}
