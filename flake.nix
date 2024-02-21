{
  description = "Flake for rust-website";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.11";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      rec {
        packages.default = pkgs.stdenv.mkDerivation {
          name = "rust-website";
          src = ./src;
          buildInputs = with pkgs; [
            rustc
          ];
          installPhase = ''
            rustc $src/rust-website.rs
            mkdir -p $out/bin/
            mv ./rust-website $out/bin
          '';
        };

        apps.default = { type = "app"; program = "${packages.default}/bin/rust-website"; };
      }
    );
}
