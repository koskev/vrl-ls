{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    {
      self,
      flake-utils,
      naersk,
      nixpkgs,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk { };
        nativeBuildInputs = with pkgs; [

          clang
          pkg-config
        ];

      in
      {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          name = "vrl-ls";
          src = ./.;

          inherit nativeBuildInputs;
          LIBCLANG_PATH = with pkgs; "${llvmPackages.libclang.lib}/lib";
        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          nativeBuildInputs =
            with pkgs;
            nativeBuildInputs
            ++ [
              cargo
              rustc
              cargo-tarpaulin
              clippy
              vector

              rust-analyzer
              bacon
              tracy
            ];
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          LIBCLANG_PATH = with pkgs; "${llvmPackages.libclang.lib}/lib";
        };
      }
    );
}
