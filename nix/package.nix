{ inputs, self, ... }:
{
  perSystem =
    { pkgs, ... }:
    let
      craneLib = inputs.crane.mkLib pkgs;
    in
    {
      # For `nix build` & `nix run`:
      packages.default = craneLib.buildPackage {
        name = "vrl-ls";
        src = self;

        nativeBuildInputs = with pkgs; [

          clang
          pkg-config
        ];
        LIBCLANG_PATH = with pkgs; "${llvmPackages.libclang.lib}/lib";
      };
    };
}
