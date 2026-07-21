{
  config,
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

  env.RUST_BACKTRACE = "1";

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

  processes.server =
    let
      port = config.processes.server.ports.server.value;
    in
    {
      ports.server.allocate = 8080;
      exec = "PORT=${toString port} HOST=127.0.0.1 cargo watch -s' cargo run -p server'";
      ready = {
        http.get = {
          port = port;
          path = "/health";
          host = "127.0.0.1";
          scheme = "http";
        };
        initial_delay = 5;
        period = 5;
        probe_timeout = 3;
        success_threshold = 1;
        failure_threshold = 3;
      };
    };

  dotenv.enable = true;

  enterTest = ''
    cargo test
  '';

  profiles.website.module = {
    languages.javascript = {
      enable = true;
      directory = "./website";
      bun = {
        enable = true;
        install.enable = true;
      };
    };
  };

  git-hooks.hooks = {
    shellcheck.enable = true;
    prettier.enable = true;
    rustfmt.enable = true;
    nixfmt.enable = true;
    clippy.enable = true;
  };

  # See full reference at https://devenv.sh/reference/options/
}
