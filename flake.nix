{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
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
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell = pkgs.mkShell.override { stdenv = pkgs.llvmPackages_19.libcxxStdenv; } {
          nativeBuildInputs = with pkgs; [
            cmake
            ninja
            gdb
            (llvmPackages_19.clang-tools.override { enableLibcxx = true; })
          ];
        };
      }
    );
}
