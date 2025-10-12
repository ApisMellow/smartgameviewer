{ pkgs, ... }:

{
  packages = [
    pkgs.cargo-watch
  ];

  languages.rust = {
    enable = true;
  };

  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };
}
