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
          fnx = inputs.fenix.packages.${system};
          # nixpkgs rustPlatform is lacking behind a bit and slow with updates
          # so we create our own here with the help of fenix, which
          # has daily updates of rust toolchains
          rustPlatform = pkgs.makeRustPlatform {
            cargo = fnx.stable.cargo;
            rustc = fnx.stable.rustc;
          };

          # read package name and version directly from the
          # `Cargo.toml` to prevent them from not being in sync
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
              # rust toolchain is not installed via rustup in nix and hence
              # doesnt support `+stable` arg
              "--skip check_code_formatting"
            ];

            meta = {
              description = "A language server implementation for WGSL and WESL";
              homepage = "https://github.com/wgsl-analyzer/wgsl-analyzer";
              license = lib.licenses.OR [
                lib.licenses.mit
                lib.licenses.apache
              ];
            };
          };
          # re-expose wgsl-analyzer as the default package of the repo
          packages.default = self'.packages.wgsl-analyzer;

          # build package as a check (this is done in CI)
          checks.default = self'.packages.default;
        };
    };
}
