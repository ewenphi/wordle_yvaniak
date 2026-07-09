{
  description = "wordle game in rust for learning";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    mydevenvs = {
      url = "github:yvaniak/mydevenvs";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  nixConfig = {
    # Adapted From: https://github.com/divnix/digga/blob/main/examples/devos/flake.nix#L4
    extra-substituters = "https://wordleyvaniak.cachix.org https://devenv.cachix.org";
    extra-trusted-public-keys = "wordleyvaniak.cachix.org-1:QIy4s3r5dMLpeOfDcu9YSdlXd14tYcYs/VM1npRMJ8M= devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.mydevenvs.flakeModules.default
        inputs.mydevenvs.devenv
      ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        {
          config,
          pkgs,
          self',
          ...
        }:
        {
          packages = import ./nix/packages.nix {
            inherit pkgs;
            inherit (inputs) crane;
          };

          apps = {
            default = {
              type = "app";
              program = "${self'.packages.default}/bin/wordle";
              meta.description = "A simple wordle app that I made for learning rust";
            };
          };

          devenv.shells.default = {
            mydevenvs = {
              rust.enable = true;
              nix = {
                enable = true;
                flake.enable = true;
                check = {
                  enable = true;
                  package = config.packages.default;
                };
              };
              tools = {
                just = {
                  enable = true;
                  pre-commit.enable = true;
                  check.enable = true;
                };
              };
              docs.check = {
                enable = true;
                package = config.packages.docs;
              };
            };

            cachix.enable = true;
            cachix.pull = "wordleyvaniak";

            enterShell = ''
              echo "shell pour wordle"
            '';
          };
        };
      flake = {
      };
    };
}
