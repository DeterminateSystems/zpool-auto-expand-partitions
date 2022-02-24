let pkgs = import <nixpkgs> {}; in pkgs.stdenv.mkDerivation {
	pname = "ztest";
	version = "0.0.0";
	buildInputs = [
		pkgs.git pkgs.gitui
		
		pkgs.cargo pkgs.rustc pkgs.gcc pkgs.cargo-edit

		pkgs.pkg-config 
		pkgs.zfs.dev
	];
}
