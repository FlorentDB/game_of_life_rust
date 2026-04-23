{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    in {
    

    devShells."x86_64-linux".default = pkgs.mkShell {
    buildInputs = with pkgs; [
	cargo rustc rustfmt clippy rust-analyzer
	pkg-config
	xorg.libX11
	xorg.libXrandr
	xorg.libXi
	xorg.libXcursor
	xorg.libXinerama
	libGL
	rand
	];
    env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };
  };
}
