{
  description = "Tshort flake";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crate2nix = {
      url = "github:kolloch/crate2nix";
      flake = false;
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };
  outputs = {
    self,
    nixpkgs,
    utils,
    rust-overlay,
    crate2nix,
    ...
  }: let
    name = "tshort";
  in
    utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            (self: super: {
              rustc = self.rust-bin.stable.latest.default;
              cargo = self.rust-bin.stable.latest.default;
            })
          ];
        };
        inherit
          (import "${crate2nix}/tools.nix" {inherit pkgs;})
          generatedCargoNix
          ;
        project =
          pkgs.callPackage
          (generatedCargoNix {
            inherit name;
            src = ./.;
          })
          {
            defaultCrateOverrides =
              pkgs.defaultCrateOverrides
              // {
                ${name} = oldAttrs:
                  {
                    inherit buildInputs nativeBuildInputs;
                  }
                  // buildEnvVars;
              };
          };
        buildInputs = with pkgs; [openssl.dev];
        nativeBuildInputs = with pkgs; [rustc cargo pkgconfig nixpkgs-fmt rust-analyzer];
        buildEnvVars = {
          PKGG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      in rec {
        packages.${name} = project.rootCrate.build;

        defaultPackage = packages.${name};

        apps.${name} = utils.lib.mkApp {
          inherit name;
          drv = packages.${name};
        };

        devShell =
          pkgs.mkShell
          {
            inherit buildInputs nativeBuildInputs;
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          }
          // buildEnvVars;
      }
    );
}
