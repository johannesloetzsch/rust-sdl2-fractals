{
  description = "icebreaker";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    crate2nix.url = "github:nix-community/crate2nix";
  };

  outputs = { self, nixpkgs, flake-utils, crate2nix, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        crateName = "fractals";

        packageBuildInputs = with pkgs; [
          SDL2
          SDL2_gfx
        ];

        inherit (import "${crate2nix}/tools.nix" { inherit pkgs; })
          generatedCargoNix;

        project = import (generatedCargoNix {
          name = crateName;
          src = ./.;
        }) {
          inherit pkgs;
          defaultCrateOverrides = pkgs.defaultCrateOverrides // {
            ${crateName} = attrs: {
              buildInputs = packageBuildInputs;
            };
          };
        };

      in
      rec {
        packages.${crateName} = project.rootCrate.build;
        defaultPackage = packages.${crateName};
        devShell = pkgs.mkShell {
          shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath (with pkgs;  [
            vulkan-loader
            libxkbcommon
          ])}"
            export RUST_SRC_PATH=${pkgs.rust.packages.stable.rustPlatform.rustLibSrc};
          '';
          inputsFrom = builtins.attrValues self.packages.${system};
          nativeBuildInputs = with pkgs; [
            cargo
            rust-analyzer
            clippy
            pkg-config
          ];

          buildInputs = packageBuildInputs;
        };
      });
}
