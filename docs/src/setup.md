# Setup

## Nix

Just add `vrl-ls.url = github:koskev/vrl-ls` as an input to your flake and use `inputs.vrl-ls.defaultPackage.${pkgs.system}` as a package.

Run `nix develop github:koskev/vrl-ls` to try it out temporarily.


## Using Cargo

```
cargo install --git https://github.com/koskev/vrl-ls
```
