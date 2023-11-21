{
  description = "Rust GPU";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix/207c664b137bf699b276481614d176b9bbe9f537";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ fenix.overlays.default ];
        pkgs = import nixpkgs { inherit overlays system; };

        toolchain = "latest";
        rustPkg = fenix.packages.${system}.${toolchain}.withComponents [
          "rust-src" 
          "rustc-dev" 
          "llvm-tools-preview"
          "cargo"
          "clippy"
          "rustc"
          "rustfmt"
        ];
      in
      {
        devShell = with pkgs; mkShell rec {

          hardeningDisable = [ "fortify" ];

          nativeBuildInputs = [ pkg-config ];
          
          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";

          # WGPU_ADAPTER_NAME = "vulkan";

          buildInputs = [
            gdb
            rustPkg
            rust-analyzer-nightly
            gcc
            spirv-tools
            libxkbcommon
            # xorg.libX11
            # xorg.libXcursor
            # xwayland
            # xorg.libXrandr
            # xorg.libXi
            vulkan-loader
            vulkan-tools
            vulkan-headers
            vulkan-validation-layers
            wayland
            libGL
          ];
        };
      });
}
