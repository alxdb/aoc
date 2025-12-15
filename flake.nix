{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    zig.url = "github:mitchellh/zig-overlay";
  };
  outputs =
    {
      self,
      nixpkgs,
      utils,
      zig,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        zigPkgs = zig.packages.${system};
      in
      {
        devShell = pkgs.mkShell { buildInputs = [ zigPkgs.default ]; };
      }
    );
}
