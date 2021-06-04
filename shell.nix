with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";

  nativeBuildInputs = [
    rustc rustfmt cargo rust-bindgen

  ];

  RUST_BACKTRACE = "1";
}
