# Setup

## Nix

Just add `vrl-ls.url = github:koskev/vrl-ls` as an input to your flake and use `inputs.vrl-ls.defaultPackage.${pkgs.system}` as a package.

Run `nix develop github:koskev/vrl-ls` to try it out temporarily.

To test a pre-configured neovim instance use `nix develop github:koskev/vrl-ls?dir=docs&ref=main`


## Using Cargo

```
cargo install --git https://github.com/koskev/vrl-ls
```
