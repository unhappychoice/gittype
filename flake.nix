{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      pname = cargoToml.package.name;
      version = cargoToml.package.version;
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system:
        f {
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
        }
      );
    in
    {
      description = cargoToml.package.description;
      packages = forAllSystems ({ pkgs }: {
        default = pkgs.rustPlatform.buildRustPackage rec {
          inherit pname version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.perl pkgs.pkg-config pkgs.git ];
          buildInputs = [ pkgs.openssl ];
          doCheck = true;
        };
      });

      devShells = forAllSystems ({ pkgs }: {
        default = pkgs.mkShell {
          buildInputs = [
            pkgs.rust-bin.stable.latest.default
            pkgs.openssl
            pkgs.pkg-config
            pkgs.perl
            pkgs.git
          ];
        };
      });

      defaultPackage = self.packages;
      defaultDevShell = self.devShells;
    };
}
