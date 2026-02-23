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
        libX11
        libXcursor
        libXi
        libXrandr
        alsa-lib
        udev
      ];

      # Robust nightly selection
      rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
        extensions = [ "rust-src" "rust-analyzer" "rustfmt" "clippy" ];
      });
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
					jless
					nickel
          pkg-config
          rustToolchain
          mold
          clang
					jq
				vscode-extensions.vadimcn.vscode-lldb
        ];

        buildInputs = runtimeLibs;

        shellHook = ''
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath runtimeLibs}"
					export CODELLDB_PATH="${pkgs.vscode-extensions.vadimcn.vscode-lldb}/share/vscode/extensions/vadimcn.vscode-lldb/adapter/codelldb"
					export LIBLLDB_PATH=${pkgs.vscode-extensions.vadimcn.vscode-lldb}/share/vscode/extensions/vadimcn.vscode-lldb/lldb/lib/liblldb.so
          export RUSTFLAGS="-C link-arg=-fuse-ld=mold -Zshare-generics=y"
					export CARGO_BUILD_JOBS=2
					export CC=clang
        '';
      };
    };
}
