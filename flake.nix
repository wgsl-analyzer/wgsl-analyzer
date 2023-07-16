{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        rust = pkgs.rust-bin.stable.latest.default;
        builder = pkgs.callPackage naersk { rustc = rust; };
        pkgs = import nixpkgs { inherit system overlays; };

      in rec {
        packages.default = builder.buildPackage {
          name = "wgsl-analyzer";
          src = ./.;
          cargoBuildOptions = opts: [ "-p" "wgsl_analyzer" ] ++ opts;
        };
        overlays.default = (self: super: { wgsl-analyzer = packages.default; });

        apps.default = packages.default;
        devShell =
          pkgs.mkShell { packages = with pkgs; [ rust-analyzer rust ]; };
      });
}
