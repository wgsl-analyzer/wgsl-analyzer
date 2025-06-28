{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        crane.url = "github:ipetkov/crane";
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = {
        self,
        nixpkgs,
        crane,
        flake-utils,
        ...
    }:
    flake-utils.lib.eachDefaultSystem (
    system: let
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (pkgs) lib stdenv;
        craneLib = crane.mkLib pkgs;
        weslFilter = path: _type: builtins.match ".*\.(wgsl|wesl)$" path != null;
        weslOrCargo = path: type:
        (weslFilter path type) || (craneLib.filterCargoSources path type);

        wgsl-analyzer = craneLib.buildPackage {
        src = lib.cleanSourceWith {
            src = craneLib.path ./.;
            filter = weslOrCargo;
        };

        buildInputs = lib.optionals stdenv.isDarwin [
            pkgs.libiconv
        ];

        cargoExtraArgs = "-p wgsl-analyzer";
        pname = "wgsl-analyzer";
        version = "0.0.0";
        };
    in {
        packages.default = wgsl-analyzer;
    }
    )
    // {
    overlays.default = final: prev: {wgsl-analyzer = self.packages.${final.system}.default;};
    };
}
