# Harbor

Harbor is an open source secrets management and distribution platform.

> Official Website: [https://harbor.maariz.org](https://harbor.maariz.org)

## Repository layout

- `crates/cli`: Rust CLI (`harbor`) for managing secrets and projects.
- `crates/server`: Axum-based HTTP service with a health endpoint.
- `crates/crypto`: ChaCha20-Poly1305 encryption helper library.
- `website/`: Bun + React + Tailwind marketing site and static build.
- `devenv.nix`, `flake.nix`: Nix/dev environment configuration.

## Core concepts

- Projects contain secrets.
- Each secret is scoped to an environment: `dev`, `staging`, or `prod`.
- Secrets are encrypted with ChaCha20-Poly1305 and stored in SQLite.
- The encryption key is stored in the OS keyring under service `harbor` and
  account `encryption-key`.

## CLI

### Build and run

```bash
cargo build -p cli
cargo run -p cli -- --help
```

### Configuration

The CLI expects a `.harbor.toml` at the repo root:

```toml
version = "1"
name = "my-project"
config = "dev"
```

You can create it manually or via the interactive setup command (requires `fzf`):

```bash
harbor setup
```

### Common commands

```bash
# Project lifecycle
harbor project create
harbor project list
harbor project delete <name>

# Manage secrets
harbor set -e dev API_KEY=secret OTHER=values
harbor show -e dev API_KEY
harbor delete -e dev API_KEY

# Run a command with secrets injected into the environment
harbor inject -e dev -- bun dev

# Open a shell with secrets injected
harbor shell -e dev

# List secrets (currently uses dev environment only)
harbor list
```

Notes:

- `set` is also available as the `add` alias.
- The default secret store lives at `~/.local/share/harbor/database.db`.
- `harbor --help` shows full usage details.

## Data model

- `projects`: `id`, `name`, `created_at`
- `secrets`: `id`, `name`, `project_id`, `config`, `secret`, `nonce`, `created_at`

## Server

The server crate is a minimal Axum app with a health check.

```bash
cargo run -p server
```

Environment variables:

- `HOST` (default `127.0.0.1`)
- `PORT` (default `8080`)
- `HARBOR_DATA_DIR` (default `/var/lib/harbor`)

Health endpoint:

- `GET /health` -> `OK`

## Website

The marketing site lives in `website/` and is built with Bun, React, and
Tailwind.

```bash
cd website
bun install
bun dev
```

Other scripts:

```bash
bun run build
bun start
```

The Dockerfile builds the static site with Bun and serves it via Caddy. The
default Caddy config serves `website/dist` on port `8080`.

## Development

```bash
cargo test
```

For Nix users, `flake.nix` + `devenv.nix` provide a dev shell and a server
process runner (see `devenv.nix` for details).

## License

GPL-3.0 (see `LICENSE`).
