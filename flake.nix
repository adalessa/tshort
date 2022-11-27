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
        sha256 = "MleotBL4T0tATZxe9ykc/PDufOaG2UdhMLspNX9I8d8=";
      };

      cargoSha256 = "e3H4v3BPfGOPPf4zpxUUgMCymdnQ6Kx6/vVR8GXz8bQ=";

      meta = with pkgs.lib; {
        description = "A cli tool to manage tmux session";
        homepage = "https://github.com/adalessa/tshort";
        license = licenses.unlicense;
        maintainers = [ maintainers.tailhook ];
      };
    };
  };
}
