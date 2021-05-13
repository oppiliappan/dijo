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
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";

        # Get a specific rust version
        mozilla = pkgs.callPackage (mozillapkgs + "/package-set.nix") { };
        chanspec = {
          date = "2021-03-31";
          channel = "nightly";
          sha256 = "oK5ebje09MRn988saJMT3Zze/tRE7u9zTeFPV1CEeLc="; # set zeros after modifying channel or date
        };

        rustChannel = mozilla.rustChannelOf chanspec;
        rust = rustChannel.rust;
        rust-src = rustChannel.rust-src;


        naersk-lib = naersk.lib."${system}".override {
          cargo = rust;
          rustc = rust;
        };
      in
      rec {
        packages.my-project = naersk-lib.buildPackage {
          pname = "dijo";
          version = "0.2.7";
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
            rust-src
            pkgs.rust-analyzer
            pkgs.cargo
            pkgs.openssl
            pkgs.ncurses
          ];
          RUST_SRC_PATH = "${rust-src}/lib/rustlib/src/rust/library";
          RUST_LOG = "info";
          RUST_BACKTRACE = 1;

        };
      });
}
