{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    recordings.url = "git+https://codeberg.org/kokev/lsp-recorder.git";
    vrl-ls.url = "..";
  };

  outputs =
    {
      self,
      flake-utils,
      nixpkgs,
      recordings,
      vrl-ls,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        baseNeovim = recordings.lib.${system}.baseNeovim.mkNeovim {
          treesitterPlugins = [ "vrl" ];
          extraConfig = ''
            vim.filetype.add({
              extension = {
                vrl = "vrl",
              }
            })

            vim.lsp.config['vrl-ls'] = {
              cmd = { "vrl-ls" },
              filetypes = { 'vrl' },
              root_markers = { '.git' },
            }
            vim.lsp.enable('vrl-ls')
          '';
        };

        nativeBuildInputs =
          with pkgs;
          [
            baseNeovim
            vrl-ls.defaultPackage.${system}

            gnumake
            mdbook
            git-cliff
            nodejs
          ]
          ++ recordings.lib.${system}.baseNeovim.nativeBuildInputs;
      in
      {
        # For `nix develop`:
        devShell = pkgs.mkShell {
          inherit nativeBuildInputs;
        };
      }
    );
}
