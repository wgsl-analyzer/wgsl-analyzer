{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      fenix,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        fnx = fenix.packages.${system};
        rust = fnx.combine [
          fnx.stable.rustc
          fnx.stable.cargo
        ];
        inherit (pkgs) lib stdenv;
        craneLib = (crane.mkLib pkgs).overrideToolchain (_: rust);
        weslFilter = path: _type: builtins.match ".*\\.(wgsl|wesl|llw|wgsl.txt|.json|.md)$" path != null;
        weslOrCargo = path: type: (weslFilter path type) || (craneLib.filterCargoSources path type);

        # cargo manifests that contain multiline inline tables.
        tomlFiles = [
          "$out/crates/profile/Cargo.toml"
          "$out/crates/project-model/Cargo.toml"
        ];

        # toml 1.0 requires inline tables ({ ... }) to stay on one line.
        fixMultilineInlineTables = ''
          :a
          # append the next input line to the pattern space.
          N
          # if this is not the last line, branch back to :a and append another.
          $!ba

          # join the line after " = {" to the opening line.
          s/ = {\n[[:space:]]*/ = { /g

          # join comma-terminated lines, keeping the comma.
          s/,\n[[:space:]]*/, /g

          # join the closing brace to the preceding line.
          s/\n[[:space:]]*}/ }/g
        '';

        allRewriteStatements = map (file: ''
          # rewrite multiline inline tables in this Cargo manifest.
          sed -i '${fixMultilineInlineTables}' "${file}"
        '') tomlFiles;

        # format the source to remove the multiline parts in the toml files since nix can't handle that yet with its default rust builder
        formatSource = ''
          cp -r ${./.} "$out"
          chmod -R u+w "$out"

          cd "$out"
          ${builtins.concatStringsSep "\n" allRewriteStatements}

          chmod -R u-w "$out"
        '';
      in
      {
        packages = {
          formattedSource = pkgs.runCommand "formatted-src" { } formatSource;
          wgsl-analyzer = craneLib.buildPackage {
            src = lib.cleanSourceWith {
              src = self.packages.${system}.formattedSource;
              filter = weslOrCargo;
            };

            buildInputs = lib.optionals stdenv.isDarwin [
              pkgs.libiconv
            ];

            cargoTestExtraArgs =
              let
                # those tests are somehow super sensitive to the local file
                # system and break for the nix builds since it
                # uses it's own file system structure for
                # builds
                ignoredTests = builtins.concatStringsSep " " (
                  map (test: "--skip ${test}") [
                    "config::tests::generate_config_documentation"
                    "config::tests::generate_package_json_config"
                  ]
                );
              in
              "-- ${ignoredTests}";

            cargoExtraArgs = "-p wgsl-analyzer";
            pname = "wgsl-analyzer";
            version = "0.0.0";
          };
          default = self.packages.${system}.wgsl-analyzer;
        };
      }
    )
    // {
      overlays.default = final: prev: { wgsl-analyzer = self.packages.${final.system}.default; };
    };
}
