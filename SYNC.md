# Template Sync

This template shares files with [rust-lib-template](https://github.com/byx-darwin/rust-lib-template).

## Shared files (keep in sync)

- `crates/core/` — domain types, error handling, SafePath
- `rust-toolchain.toml`, `rustfmt.toml`, `.cargo/config.toml`
- `deny.toml`, `.pre-commit-config.yaml`
- `_typos.toml`, `.tokeignore`, `.gitignore`
- `.env.example`, `LICENSE.md`, `SECURITY.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`
- `docs/pre-commit-usage.md`, `docs/release.md`

## Divergent files (intentionally different)

- `Cargo.toml` — workspace deps differ (ratatui vs clap)
- `clippy.toml` — TUI uses sync I/O rules
- `Makefile` — TUI-specific targets (demo, run)
- `apps/` — TUI binary instead of CLI
- `docs/` — tui-patterns.md vs cli-patterns.md
- `CLAUDE.md`, `README.md`, `cargo-generate.toml`

## Sync Process

1. Make the change in one template
2. Run `scripts/sync.sh <path-to-other-template> check`
3. Apply with `scripts/sync.sh <path-to-other-template> apply`
4. Commit with `sync: port <change> from <source>`
