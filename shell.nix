let pkgs = import <nixpkgs> {}; in pkgs.stdenv.mkDerivation {
	pname = "ztest";
	version = "0.0.0";
	buildInputs = [
		pkgs.zfs.dev
		pkgs.pkg-config 
		pkgs.cargo pkgs.rustc pkgs.gcc pkgs.cargo-edit
	];
}
