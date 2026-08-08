{
  cmake,
  fetchurl,
  llvmPackages,
  openssl,
  libcap ? null,
  rustPlatform,
  pkg-config,
  lib,
  stdenv,
  version ? "0.0.0",
  ...
}:
let
  rustyV8ReleaseUrl = "https://github.com/openai/codex/releases/download/rusty-v8-v150.4.0";
  rustyV8 = lib.optionalAttrs (stdenv.hostPlatform.system == "x86_64-linux") {
    archive = fetchurl {
      url = "${rustyV8ReleaseUrl}/librusty_v8_ptrcomp_sandbox_release_x86_64-unknown-linux-gnu.a.gz";
      hash = "sha256-o1x10fJuapg4haRbM0kKTr5U8FBQVosyuJz7QhswtYM=";
    };
    binding = fetchurl {
      url = "${rustyV8ReleaseUrl}/src_binding_ptrcomp_sandbox_release_x86_64-unknown-linux-gnu.rs";
      hash = "sha256-dyeCauR5vbZF6Acjn7EtH44uI956bPFvXuWSaQ0dhQY=";
    };
  };
in
rustPlatform.buildRustPackage (_: {
  env = {
    PKG_CONFIG_PATH = lib.makeSearchPathOutput "dev" "lib/pkgconfig" (
      [ openssl ] ++ lib.optionals stdenv.isLinux [ libcap ]
    );
  }
  // lib.optionalAttrs (stdenv.hostPlatform.system == "x86_64-linux") {
    RUSTY_V8_ARCHIVE = rustyV8.archive;
    RUSTY_V8_SRC_BINDING_PATH = rustyV8.binding;
  };
  pname = "codex-rs";
  inherit version;
  cargoLock.lockFile = ./Cargo.lock;
  doCheck = false;
  src = ./.;

  # Patch the workspace Cargo.toml so that cargo embeds the correct version in
  # CARGO_PKG_VERSION (which the binary reads via env!("CARGO_PKG_VERSION")).
  # On release commits the Cargo.toml already contains the real version and
  # this sed is a no-op.
  postPatch = ''
    sed -i 's/^version = "0\.0\.0"$/version = "${version}"/' Cargo.toml
  '';
  nativeBuildInputs = [
    cmake
    llvmPackages.clang
    llvmPackages.libclang.lib
    openssl
    pkg-config
  ]
  ++ lib.optionals stdenv.isLinux [
    libcap
  ];

  cargoLock.outputHashes = {
    "crossterm-0.29.0" = "sha256-ewiWWQPEU1lSUHzmZTiO5yes5luIaQ9TrvCNnTWhxpE=";
    "nucleo-0.5.0" = "sha256-Hm4SxtTSBrcWpXrtSqeO0TACbUxq3gizg1zD/6Yw/sI=";
    "nucleo-matcher-0.3.1" = "sha256-Hm4SxtTSBrcWpXrtSqeO0TACbUxq3gizg1zD/6Yw/sI=";
    "runfiles-0.1.0" = "sha256-uJpVLcQh8wWZA3GPv9D8Nt43EOirajfDJ7eq/FB+tek=";
    "tokio-tungstenite-0.28.0" = "sha256-hJAkvWxDjB9A9GqansahWhTmj/ekcelslLUTtwqI7lw=";
    "tungstenite-0.27.0" = "sha256-AN5wql2X2yJnQ7lnDxpljNw0Jua40GtmT+w3wjER010=";
  };

  meta = with lib; {
    description = "OpenAI Codex command‑line interface rust implementation";
    license = licenses.asl20;
    homepage = "https://github.com/openai/codex";
    mainProgram = "codex";
  };
})
