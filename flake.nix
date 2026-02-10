{
	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
		rust-overlay.url = "github:oxalica/rust-overlay";
	};

	outputs = { self, nixpkgs, rust-overlay }:
		let
			system = "x86_64-linux";
			overlays = [ (import rust-overlay) ];
			pkgs = import nixpkgs { inherit system overlays; };

			runtimeLibs = with pkgs; [
				libxkbcommon
				vulkan-loader
				wayland
				xorg.libX11
				xorg.libXcursor
				xorg.libXi
				xorg.libXrandr
				alsa-lib
				udev
			];
		in
			{
			devShells.${system}.default = pkgs.mkShell {
				packages = with pkgs; [
					pkg-config
					(rust-bin.stable.latest.default.override {
						extensions = [ "rust-src" "rust-analyzer" "rustfmt" "clippy" ];
					})
					mold
					clang
				];

				buildInputs = runtimeLibs;

				shellHook = ''
					export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath runtimeLibs}"
					export RUSTFLAGS="-C link-arg=-fuse-ld=mold -Zshare-generics=y"
					'';
			};
		};
}
