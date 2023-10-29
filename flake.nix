{
  description = "Development environment flake for geo-svg";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/23.05";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-utils, nixpkgs, ... }@inputs: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; config.allowUnfree = true; };
      fenix-pkgs = with inputs.fenix.packages.${system};
        combine [
          stable.cargo
          stable.rustc
          stable.rust-src
          stable.rust-analyzer
          stable.clippy
          complete.rustfmt
        ];
    in
    {
      devShells = rec {
        default = with pkgs; mkShell rec {
          buildInputs = [
            fenix-pkgs
            clang
            openssl
            pkgconfig
          ];
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        }
        ;
      };
    }
  );
}
