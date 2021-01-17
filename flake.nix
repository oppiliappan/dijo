{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
    mozillapkgs = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, naersk, mozillapkgs }:
  utils.lib.eachDefaultSystem (system: let
    pkgs = nixpkgs.legacyPackages."${system}";

      # Get a specific rust version
      mozilla = pkgs.callPackage (mozillapkgs + "/package-set.nix") {};
      rust = (mozilla.rustChannelOf {
        date = "2020-12-23";
        channel = "nightly";
        sha256 = "LbKHsCOFXWpg/SEyACfzZuWjKbkXdH6EJKOPSGoO01E="; # set zeros after modifying channel or date
      }).rust;

      naersk-lib = naersk.lib."${system}".override {
        cargo = rust;
        rustc = rust;
      };
    in rec {
      packages.my-project = naersk-lib.buildPackage {
        pname = "dijo";
        root = ./.;
      };
      defaultPackage = packages.my-project;
      apps.my-project = utils.lib.mkApp {
        drv = packages.my-project;
      };
      defaultApp = apps.my-project;
      devShell = pkgs.mkShell {
        nativeBuildInputs = [
          rust
          pkgs.cargo
          pkgs.cargo
          pkgs.ncurses
        ];
      };
    });
  }
