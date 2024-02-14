{
  description = "Development environment flake for geo-svg";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/23.11";
    flake-utils.url = "github:numtide/flake-utils";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ { flake-parts, flake-utils, fenix, ... }:
    flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      {
        systems = flake-utils.lib.defaultSystems;
        perSystem = { pkgs, lib, system, ... }:
          let
            fnx = fenix.packages.${system};
            rust-fenix = fnx.combine [
              fnx.stable.cargo
              fnx.stable.rustc
              fnx.stable.rust-src
              fnx.stable.rust-analyzer
              fnx.stable.clippy
              fnx.complete.rustfmt
            ];
          in
          {
            devShells.default = pkgs.mkShell rec {
              buildInputs = [
                rust-fenix
                pkgs.clang
                pkgs.openssl
                pkgs.pkg-config
              ];
              LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
            };
          };
      };
}
