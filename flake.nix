{
  description = "Tshort flake";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };
  outputs = { self, nixpkgs }:
    let
    system = "x86_64-linux";
  pkgs = nixpkgs.legacyPackages.${system};
  in
  {
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage rec {
      pname = "tshort";
      version = "v0.1.4";

      src = pkgs.fetchFromGitHub {
        owner = "adalessa";
        repo = pname;
        rev = version;
        sha256 = "gndbCJNGtL/5Zl/FHNR3tIlbHZpPm+suQKLXCLIgc4A=";
      };

      cargoSha256 = "mt6AN0qokVzhq6xN8mLmgQ+nwYDA/sibGK1b/VQz+fY=";

      meta = with pkgs.lib; {
        description = "A cli tool to manage tmux session";
        homepage = "https://github.com/adalessa/tshort";
        license = licenses.unlicense;
        maintainers = [ maintainers.tailhook ];
      };
    };
  };
}
