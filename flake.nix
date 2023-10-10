{
  description = "zpool_part_disks";

  inputs.nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1.533189.tar.gz";

  inputs.flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.0.1.tar.gz";

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
