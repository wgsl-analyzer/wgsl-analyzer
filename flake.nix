{
  description = "wgsl-analyzer: A language server implementation for WGSL and WESL";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        {
          self',
          pkgs,
          lib,
          system,
          ...
        }:
        let
          fenix = inputs.fenix.packages.${system};
          # nixpkgs's `rustPlatform` lags behind a bit, so create one with fenix.
          # Fenix has daily updates of rust toolchains.
          rustPlatform = pkgs.makeRustPlatform {
            cargo = fenix.stable.cargo;
            rustc = fenix.stable.rustc;
          };

          # read package name and version directly from the `Cargo.toml` to prevent them from not being in sync
          cargoToml = fromTOML (builtins.readFile ./crates/wgsl-analyzer/Cargo.toml);
          version = cargoToml.package.version;
          pname = cargoToml.package.name;
        in
        {
          packages.wgsl-analyzer = rustPlatform.buildRustPackage {
            inherit pname version;

            src = lib.sourceFilesBySuffices ./. [
              # rust related file types
              ".rs"
              ".toml"
              ".lock"
              # shader related file types
              ".wesl"
              ".wgsl"
              # used in `parser` build.rs
              ".llw"
              # licenses
              "LICENSE-MIT"
              "LICENSE-APACHE"
              # configs, used in tests
              ".json"
              ".md"
            ];

            cargoLock.lockFile = ./Cargo.lock;

            useNextest = true;
            checkFlags = [
              # rust toolchain is not installed from rustup in nix, so `+stable` argument is not supported
              "--skip check_code_formatting"
            ];

            cargoBuildFlags = [
              "--package"
              "wgsl-analyzer"
            ];

            meta = {
              description = "A language server implementation for WGSL and WESL";
              homepage = "https://github.com/wgsl-analyzer/wgsl-analyzer";
              license = lib.licenses.OR [
                lib.licenses.mit
                lib.licenses.asl20
              ];
            };
          };
          # reëxpose wgsl-analyzer as the default package of the repo
          packages.default = self'.packages.wgsl-analyzer;

          # Check that the package builds, without running its test suite.
          checks.default = self'.packages.default.overrideAttrs {
            doCheck = false;
          };
        };
    };
}
