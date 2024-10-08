{
  description = "SDF Builder";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix/5c3ff469526a6ca54a887fbda9d67aef4dd4a921";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
    fenix,
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];
      perSystem = {
        pkgs,
        system,
        ...
      }: let
        rustPkg = fenix.packages.${system}.latest.withComponents [
          "rust-src"
          "rustc-dev"
          "llvm-tools-preview"
          "cargo"
          "clippy"
          "rustc"
          "rustfmt"
          "rust-analyzer"
        ];
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustPkg;
          rustc = rustPkg;
        };
        buildInputs = with pkgs; [
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          vulkan-loader
          vulkan-tools
          wayland
          libxkbcommon
        ];
        shadersCompilePath = "$HOME/.cache/rust-gpu-shaders";
        sdf-builder = rustPlatform.buildRustPackage {
          pname = "sdf-builder";
          version = "0.0.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          cargoLock.outputHashes = {
            "rustc_codegen_spirv-0.9.0" = "sha256-BaWGJQjbAH5WdXul2M2C1hsfsH659qpQEeM/c5bkYds=";
          };
          dontCargoSetupPostUnpack = true;
          postUnpack = ''
            mkdir -p .cargo
            cat "$cargoDeps"/.cargo/config | sed "s|cargo-vendor-dir|$cargoDeps|" >> .cargo/config
            # HACK(eddyb) bypass cargoSetupPostPatchHook.
            export cargoDepsCopy="$cargoDeps"
          '';
          nativeBuildInputs = [pkgs.makeWrapper];
          configurePhase = ''
            export SHADERS_DIR="$out/repo/shaders"
            export SHADERS_TARGET_DIR=${shadersCompilePath}
          '';
          fixupPhase = ''
            cp -r . $out/repo
            wrapProgram $out/bin/runner \
              --set LD_LIBRARY_PATH $LD_LIBRARY_PATH:$out/lib:${nixpkgs.lib.makeLibraryPath buildInputs} \
              --set PATH $PATH:${nixpkgs.lib.makeBinPath [rustPkg]}
          '';
        };
      in rec {
        packages.default = pkgs.writeShellScriptBin "sdf-builder" ''
          export CARGO_TARGET_DIR="${shadersCompilePath}"
          exec -a "$0" "${sdf-builder}/bin/runner" "$@"
        '';
        apps.default = {
          type = "app";
          program = "${packages.default}/bin/sdf-builder";
        };
        devShells.default = with pkgs;
          mkShell {
            nativeBuildInputs = [rustPkg];
            LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          };
      };
    };
}
