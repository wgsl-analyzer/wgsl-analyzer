{
  description = "wgsl-analyzer: A language server implementation for WGSL and WESL";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
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
          # nixpkg's `rustPlatform` lags behind a bit, so create one with fenix.
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

            src = ./.;

            cargoLock.lockFile = ./Cargo.lock;

            useNextest = true;
            checkFlags = [
              # rust toolchain is not installed from rustup in nix, so `+stable` argument is not supported
              "--skip check_code_formatting"
            ];

            # the xtask binary ends up in the package, but it is just a development tool
            postInstall = ''
              rm -f $out/bin/xtask
            '';

            meta = {
              description = "A language server implementation for WGSL and WESL";
              homepage = "https://github.com/wgsl-analyzer/wgsl-analyzer";
              license = lib.licenses.OR [
                lib.licenses.mit
                lib.licenses.apache
              ];
            };
          };
          # reëxpose wgsl-analyzer as the default package of the repo
          packages.default = self'.packages.wgsl-analyzer;

          # build package as a check (this is done in CI)
          checks.default = self'.packages.default;
        };
    };
}
