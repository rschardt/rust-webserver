{
  description = "Flake for rust-webserver";

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
        packages = rec {
          default = rust-webserver;
          rust-webserver = pkgs.stdenv.mkDerivation {
            name = "rust-webserver";
            src = ./src;
            buildInputs = with pkgs; [
              rustc
            ];
            installPhase = ''
                rustc $src/rust-webserver.rs
                mkdir -p $out/bin/
                mv ./rust-webserver $out/bin
            '';
          };
        };

        apps.default = { type = "app"; program = "${packages.default}/bin/rust-webserver"; };
      }
    );
}
