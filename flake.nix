# This file is part of devops-dreamland-app
#
# Copyright (c) 2023 Thomas Himmelstoss
#
# This software is subject to the MIT license. You should have
# received a copy of the license along with this program.

{
  description = "Build devops-dreamland-app crate";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, crane, ... }:
    let
      supportedSystems = [ "x86_64-linux" ];

      imageName = "devops-dreamland-app";

      forSupportedSystems = generator:
        let
          generateForSystem = system: generator rec {
            inherit system;
            pkgs = nixpkgs.legacyPackages.${system};
            craneLib = crane.mkLib pkgs;
          };
        in
        nixpkgs.lib.genAttrs supportedSystems generateForSystem;
    in
    {
      packages = forSupportedSystems ({ system, pkgs, craneLib }:
        {
          package = craneLib.buildPackage {
            src = craneLib.cleanCargoSource (craneLib.path ./.);

            strictDeps = true;

            buildInputs = with pkgs.lib; [ ]
              ++ optional pkgs.stdenv.isDarwin pkgs.libiconv;

            meta = with pkgs.lib; {
              description = "A simple axum application";
              license = licenses.mit;
              platforms = platforms.linux;
              mainProgram = "devops-dreamland-app";
            };
          };

          image = pkgs.dockerTools.streamLayeredImage {
            name = imageName;

            # Use the commit date to get a reproducible build with a
            # more helpful build date. See:
            # https://nixos.wiki/wiki/Docker#Reproducible_image_dates
            created = builtins.substring 0 8 self.lastModifiedDate;

            config = {
              Cmd = [ (pkgs.lib.getExe self.packages.${system}.package) ];
            };
          };

          default = self.packages.${system}.image;
        });

      devShells = forSupportedSystems ({ system, pkgs, craneLib, ... }:
        let
          fix = pkgs.writeShellScriptBin "fix" ''
            cargo fmt
            cargo clippy --fix --allow-dirty --allow-staged --all-targets
          '';

          checkFmt = pkgs.writeShellScriptBin "chkfmt" ''
            cargo fmt --check
          '';

          lint = pkgs.writeShellScriptBin "lint" ''
            cargo clippy --all-targets -- --deny warnings
          '';

          loadImage = pkgs.writeShellScriptBin "load-image-with-tag" ''
            IMAGE_LOADER_SCRIPT=$(nix build --print-out-paths .#image)
            $IMAGE_LOADER_SCRIPT --repo_tag "''${1:-app:latest}" | docker load
          '';
        in
        {
          default = craneLib.devShell {
            inputsFrom = [ self.packages.${system}.package ];

            # This environment variable is required by rust-analyzer
            # to find the source and expand proc macros. See:
            # https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

            packages = with pkgs; [
              fix
              checkFmt
              lint
              loadImage
              cocogitto
              cargo-edit
            ];
          };
        });

      formatter = forSupportedSystems ({ pkgs, ... }: pkgs.nixpkgs-fmt);
    };
}
