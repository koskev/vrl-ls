_: {
  perSystem =
    { pkgs, ... }:
    {
      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          clang
          pkg-config
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
    };
}
