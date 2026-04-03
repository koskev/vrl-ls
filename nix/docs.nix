{ inputs, ... }:
{
  perSystem =
    {
      pkgs,
      system,
      self',
      ...
    }:
    let
      baseNeovim = inputs.recordings.lib.${system}.baseNeovim.mkNeovim {
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
          self'.packages.default

          gnumake
          mdbook
          git-cliff
          nodejs
        ]
        ++ inputs.recordings.lib.${system}.baseNeovim.nativeBuildInputs;
    in
    {
      devShells = {
        docs = pkgs.mkShell {
          inherit nativeBuildInputs;
        };
      };
    };
}
