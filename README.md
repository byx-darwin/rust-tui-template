# {{ project-name }}

A Rust terminal user interface (TUI) built from the [rust-tui-template](https://github.com/byx-darwin/rust-tui-template).

## Quickstart

```bash
# Install
cargo install --path apps/tui

# Launch the TUI
{{ project-name }}-tui

# Launch with simulated data
{{ project-name }}-tui --demo
```

## Development

```bash
# Install dev tools
make install-tools

# Build and test
make build
make test
make lint

# Run locally
make run

# Run with demo data
make demo
```

## Terminal Requirements

| Feature       | Minimum                          |
|---------------|----------------------------------|
| True color    | Recommended (COLORTERM=truecolor)|
| Terminal size | 80x24 or larger                  |
| Unicode       | Required for border glyphs       |

{{ project-name }} respects `NO_COLOR` and degrades gracefully.

## License

MIT — see [LICENSE.md](LICENSE.md).
