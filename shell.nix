let pkgs = import <nixpkgs> {}; in pkgs.stdenv.mkDerivation {
	pname = "ztest";
	version = "0.0.0";
	buildInputs = builtins.attrValues {
		inherit (pkgs)
			git gitui

			cargo rustc gcc cargo-edit

			pkg-config;
	} ++ [
		pkgs.zfs.dev
		pkgs.util-linux.dev
	];
}
