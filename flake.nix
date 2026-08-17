{
  description = "Development Nix flake for OpenAI Codex CLI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems f;

      # Read the version from the workspace Cargo.toml (the single source of
      # truth used by the release workflow).
      cargoToml = builtins.fromTOML (builtins.readFile ./codex-rs/Cargo.toml);
      cargoVersion = cargoToml.workspace.package.version;

      # When building from a release commit the Cargo.toml already carries the
      # real version (e.g. "0.101.0").  On the main branch it is the placeholder
      # "0.0.0", so we fall back to a dev version derived from the flake source.
      version =
        if cargoVersion != "0.0.0"
        then cargoVersion
        else "0.0.0-dev+${self.shortRev or "dirty"}";
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          codex-rs = pkgs.callPackage ./codex-rs {
            inherit version;
            rustPlatform = pkgs.makeRustPlatform {
              cargo = pkgs.rust-bin.stable."1.95.0".minimal;
              rustc = pkgs.rust-bin.stable."1.95.0".minimal;
            };
          };
        in
        {
          codex-rs = codex-rs;
          default = codex-rs;
        }
        // nixpkgs.lib.optionalAttrs (system == "x86_64-linux") {
          # Proposal-only discovery. Its output is explicitly non-authorizing;
          # the full check consumes a separately reviewed frozen inventory.
          hepta-workspace-check-discovery = pkgs.callPackage ./nix/hepta-workspace-check.nix {
            product = codex-rs;
            discoveryOnly = true;
          };
        }
      );

      # This is deliberately an independent output. The product derivation
      # has doCheck=false; aliasing it here would not execute or inventory any
      # tests and would make the product and check store paths identical.
      checks.x86_64-linux =
        let
          pkgs = import nixpkgs {
            system = "x86_64-linux";
            overlays = [ rust-overlay.overlays.default ];
          };
        in
        {
          workspace = pkgs.callPackage ./nix/hepta-workspace-check.nix {
            product = self.packages.x86_64-linux.codex-rs;
            expectedInventory = ./nix/hepta-expected-check-inventory-v1.json;
          };
        };

      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          rust = pkgs.rust-bin.stable."1.95.0".default.override {
            extensions = [ "rust-src" "rust-analyzer" ];
          };
        in
        {
          default = pkgs.mkShell {
            buildInputs = [
              rust
              pkgs.pkg-config
              pkgs.openssl
              pkgs.cmake
              pkgs.llvmPackages.clang
              pkgs.llvmPackages.libclang.lib
            ];
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            # Use clang for BoringSSL compilation (avoids GCC 15 warnings-as-errors)
            shellHook = ''
              export CC=clang
              export CXX=clang++
            '';
          };
        }
      );
    };
}
