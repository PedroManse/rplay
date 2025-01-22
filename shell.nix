let
	nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.11";
	pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShellNoCC {
	packages = with pkgs; [
		sqlx-cli
		pkg-config
		openssl
		sqlite
	];

	COMPUTER_NAME="dev-rplay";
	PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
