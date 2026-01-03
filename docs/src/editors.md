# Editors

# Neovim

Add the following to your config
```lua
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

```

# (Evil-)Helix

# VSCodium

# IntelliJ

Just don't. IntelliJ is probably nice for Java development, but for other languages it just seems rather bad and does not have native support LSP, DAP, or Treesitter.

To get basic support you can use [lsp4ij](https://github.com/redhat-developer/lsp4ij). However, you either need to separately install syntax highlighting or use the slow Treesitter bridge of the language server.
