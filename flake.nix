{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };
  outputs =
    {
      self,
      nixpkgs,
      utils,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              cabal-install
              ghc
              hpack
              # https://github.com/haskell/haskell-language-server/issues/176
              haskellPackages.hspec-discover
              # For http support in haskell
              zlib
            ];
          };
      }
    );
}
