{
  pkgs,
  ...
}:

{
  packages = with pkgs; [
    cargo-watch
    prettier
    nixfmt
    clippy
    sqlite
  ];

  languages.rust = {
    enable = true;
    lsp.enable = true;
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
    ];
  };

  dotenv.enable = true;

  enterTest = ''
    cargo test
  '';

  git-hooks.hooks = {
    shellcheck.enable = true;
    prettier.enable = true;
    rustfmt.enable = true;
    nixfmt.enable = true;
    clippy.enable = true;
  };

  # See full reference at https://devenv.sh/reference/options/
}
