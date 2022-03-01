let pkgs = import <nixpkgs> { }; inherit (pkgs) lib; in
pkgs.stdenv.mkDerivation {
  pname = "ztest";
  version = "0.0.0";

  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  C_INCLUDE_PATH = lib.makeSearchPathOutput "dev" "include" [ pkgs.util-linux pkgs.llvmPackages.clang ];
  BINDGEN_EXTRA_CLANG_ARGS = lib.concatStringsSep " " [
    (builtins.readFile "${pkgs.stdenv.cc}/nix-support/libc-crt1-cflags")
    (builtins.readFile "${pkgs.stdenv.cc}/nix-support/libc-cflags")
    (builtins.readFile "${pkgs.stdenv.cc}/nix-support/cc-cflags")
    (builtins.readFile "${pkgs.clang}/nix-support/libc-crt1-cflags")
    (builtins.readFile "${pkgs.clang}/nix-support/libc-cflags")
    (builtins.readFile "${pkgs.clang}/nix-support/cc-cflags")
    (lib.optionalString pkgs.stdenv.cc.isClang "-idirafter ${pkgs.stdenv.cc.cc}/lib/clang/${lib.getVersion pkgs.stdenv.cc.cc}/include")
    (lib.optionalString pkgs.stdenv.cc.isGNU (lib.concatStringsSep " " [
      "-isystem ${pkgs.stdenv.cc.cc}/include/c++/${lib.getVersion pkgs.stdenv.cc.cc}"
      "-isystem ${pkgs.stdenv.cc.cc}/include/c++/${lib.getVersion pkgs.stdenv.cc.cc}/${pkgs.stdenv.hostPlatform.config}"
      "-idirafter ${pkgs.stdenv.cc.cc}/lib/gcc/${pkgs.stdenv.hostPlatform.config}/${lib.getVersion pkgs.stdenv.cc.cc}/include"
    ]))
  ];
  buildInputs = builtins.attrValues
    {
      inherit (pkgs)
        git gitui cloud-utils

        cargo rustc gcc gdb llvm cargo-edit

        pkg-config;
    } ++ [
    pkgs.zfs.dev
    pkgs.util-linux.dev
  ];
}
