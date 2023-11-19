{
  description = "zpool_part_disks";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  inputs.flake-compat = {
    url = github:edolstra/flake-compat;
    flake = false;
  };

  outputs =
    { self
    , nixpkgs
    , ...
    } @ inputs:
    let
      nameValuePair = name: value: { inherit name value; };
      genAttrs = names: f: builtins.listToAttrs (map (n: nameValuePair n (f n)) names);
      allSystems = [ "x86_64-linux" "aarch64-linux" "i686-linux" ];

      forAllSystems = f: genAttrs allSystems (system: f {
        inherit system;
        pkgs = import nixpkgs { inherit system; };
      });

      inherit (nixpkgs)
        lib;
    in
    {
      devShell = forAllSystems ({ system, pkgs, ... }:
        pkgs.mkShell ({
          name = "zpool_part_disks";

          inputsFrom = [
            self.packages.${system}.zpool_part_disks
          ];

          buildInputs = with pkgs; [
            cargo
            codespell
            git
            nixpkgs-fmt
            rustfmt
          ];
        }));

      packages = forAllSystems
        ({ system, pkgs, ... }:
          {
            zpool_part_disks = let cargo = builtins.fromTOML (builtins.readFile ./Cargo.toml); in
              pkgs.rustPlatform.buildRustPackage (rec {
                pname = cargo.package.name;
                version = cargo.package.version;

                src = self;

                cargoLock.lockFile = ./Cargo.lock;
                cargoLock.outputHashes = {
                  "libzfs-0.6.16" = "sha256-kQunP/xW1Zb1q+TcgAkmZkt1yDnJo9CwF5qldikVN94=";
                };

                preBuild = ''
                  substituteInPlace src/grow.rs \
                    --replace '"growpart"' '"${pkgs.cloud-utils}/bin/growpart"'
                  substituteInPlace src/lsblk.rs \
                    --replace '"lsblk"' '"${pkgs.util-linux}/bin/lsblk"'
                '';

                nativeBuildInputs = [
                  pkgs.pkg-config
                  pkgs.rustPlatform.bindgenHook
                ];

                buildInputs = [
                  pkgs.util-linux
                  pkgs.zfs.dev
                ];
              });
          });

      defaultPackage = forAllSystems ({ system, ... }: self.packages.${system}.zpool_part_disks);
    };
}
