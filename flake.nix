{
  description = "zpool-auto-expand-partitions";

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

      bindgenEnvs = pkgs: {
        # https://hoverbear.org/blog/rust-bindgen-in-nix/
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        C_INCLUDE_PATH = lib.makeSearchPathOutput "dev" "include" [ pkgs.util-linux ];
        BINDGEN_EXTRA_CLANG_ARGS = lib.concatStringsSep " " [
          (builtins.readFile "${pkgs.stdenv.cc}/nix-support/libc-crt1-cflags")
          (builtins.readFile "${pkgs.stdenv.cc}/nix-support/libc-cflags")
          (builtins.readFile "${pkgs.stdenv.cc}/nix-support/cc-cflags")
          (lib.optionalString pkgs.stdenv.cc.isGNU (lib.concatStringsSep " " [
            "-isystem ${pkgs.stdenv.cc.cc}/include/c++/${lib.getVersion pkgs.stdenv.cc.cc}"
            "-isystem ${pkgs.stdenv.cc.cc}/include/c++/${lib.getVersion pkgs.stdenv.cc.cc}/${pkgs.stdenv.hostPlatform.config}"
            "-idirafter ${pkgs.stdenv.cc.cc}/lib/gcc/${pkgs.stdenv.hostPlatform.config}/${lib.getVersion pkgs.stdenv.cc.cc}/include"
          ]))
        ];
      };
    in
    {
      devShell = forAllSystems ({ system, pkgs, ... }:
        pkgs.mkShell ({
          name = "zpool-auto-expand-partitions";

          inputsFrom = [
            self.packages.${system}.zpool-auto-expand-partitions
          ];

          buildInputs = with pkgs; [
            cargo
            codespell
            nixpkgs-fmt
            rustfmt
            git
          ];
        } // bindgenEnvs pkgs));

      packages = forAllSystems
        ({ system, pkgs, ... }:
          {
            zpool-auto-expand-partitions = let cargo = builtins.fromTOML (builtins.readFile ./Cargo.toml); in
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
                ];

                buildInputs = [
                  pkgs.zfs.dev
                ];
              } // bindgenEnvs pkgs);
          });

      defaultPackage = forAllSystems ({ system, ... }: self.packages.${system}.zpool-auto-expand-partitions);
    };
}
