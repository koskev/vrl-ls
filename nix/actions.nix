{ inputs, ... }:
let
  inherit (inputs.nix-actions.lib) steps;
  inherit (inputs.nix-actions.lib) platforms;
  inherit (inputs.nix-actions.lib) mkCachixSteps;
  inherit (inputs.nix-actions.lib) commonSteps;
  inherit (inputs.nix-actions.lib) actions;
in
{
  imports = [ inputs.actions-nix.flakeModules.default ];
  flake.actions-nix = {
    pre-commit.enable = true;
    defaultValues = {
      jobs = {
        runs-on = "ubuntu-latest";
      };
    };
    workflows = {
      ".github/workflows/docs.yaml" = {
        on = {
          push.branches = [ "main" ];
        };
        jobs = {
          docs.steps = commonSteps ++ [
            {
              run = "cd docs && nix develop ..#docs --command make build && cd ..";
            }
            {
              name = "Upload artifacts for pages";
              uses = actions.upload-pages-artifacts;
              "with".path = "docs/book";
            }
          ];
          pages = {
            permissions = {
              id-token = "write";
              pages = "write";
            };
            environment = {
              name = "github-pages";
              url = "\${{steps.deployment.outputs.page_url}}";
            };
            needs = [ "docs" ];
            steps = [
              {
                name = "Deploy to GitHub Pages";
                id = "deployment";
                uses = actions.deploy-pages;
              }
            ];
          };
        };
      };
      ".github/workflows/build.yaml" = {
        on = {
          push = { };
          pull_request = { };
        };
        env = {
          CARGO_TERM_COLOR = "always";
        };
        jobs = {
          nix-build = {
            strategy.matrix.platform = [
              platforms.linux
              platforms.linux_aarch64
              platforms.mac
            ];
            runs-on = "\${{ matrix.platform.runs-on }}";
            steps = [
              steps.checkout
              steps.installNix
              {
                name = "Build";
                run = "nix build .";
              }
            ]
            ++ mkCachixSteps { };
          };
        };
      };
    };
  };
}
