{
  description = "HumanShell - A human-friendly shell interface";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    let
      # Define the systems we want to support
      systems = [
        "x86_64-linux"   # Linux x86
        "aarch64-linux"  # Linux Arm64
        "x86_64-darwin"  # macOS x86
        "aarch64-darwin" # macOS Arm64
      ];

      # Helper function to create outputs for each system
      forAllSystems = f: flake-utils.lib.eachSystem systems f;
    in

    forAllSystems (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };

        buildInputs = with pkgs; [
          rustToolchain
        ] ++ lib.optionals stdenv.isDarwin [
          libiconv
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

      in
      {
        packages = {
          default = pkgs.rustPlatform.buildRustPackage rec {
            pname = "hs";
            version = "1.0.0";

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            inherit buildInputs nativeBuildInputs;

            # Configure Rust to build for the current system using the non-deprecated approach
            CARGO_BUILD_TARGET = pkgs.stdenv.hostPlatform.rust.rustcTargetSpec;

            # For cross compilation:
            # Enable static linking when possible for better portability
            CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUSTFLAGS = [ "-C" "target-feature=+crt-static" ];
            CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS = [ "-C" "target-feature=+crt-static" ];

            # Add necessary build flags for all targets
            preBuild = ''
              # Set up cross-compilation environment
              ${pkgs.lib.optionalString (pkgs.stdenv.hostPlatform != pkgs.stdenv.buildPlatform) ''
                export CARGO_BUILD_TARGET=${pkgs.stdenv.hostPlatform.rust.rustcTargetSpec}
                export TARGET_CC=$CC
              ''}
            '';

            meta = with pkgs.lib; {
              description = "A human-friendly shell interface";
              homepage = "https://github.com/juanparati/humanshell";
              license = licenses.mit;
              maintainers = [ ];
              platforms = with platforms; [
                "x86_64-linux"
                "aarch64-linux"
                "x86_64-darwin"
                "aarch64-darwin"
              ];
            };
          };
        };

        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          shellHook = ''
            echo "HumanShell development environment"
            echo "Rust toolchain: $(rustc --version)"
          '';
        };

        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };
      });
}