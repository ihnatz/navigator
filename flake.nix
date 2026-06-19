{
  description = "The flight of the Navigator";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-25.11";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      with nixpkgs.legacyPackages.${system};
      {
        packages.default = callPackage ./default.nix { };
        devShells.default = callPackage ./shell.nix { };
      }
    );
}