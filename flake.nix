{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      self,
      fenix,
      utils,
      nixpkgs,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        fenixPkgs = fenix.packages.${system};
      in
      {
        devShells = {
          default = pkgs.mkShell rec {
            nativeBuildInputs = [
              (fenixPkgs.default.withComponents [
                "cargo"
                "clippy"
                "rustc"
                "rustfmt"
              ])
              pkgs.pkg-config
            ];
            buildInputs = [ pkgs.openssl ];
            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
          };
        };
      }
    );
}
