# System Monitoring Panel

The template includes a system monitoring panel as a working TUI demo.

## Tabs

| Tab | Description |
|-----|-------------|
| **Overview** | CPU usage gauge + sparkline history, memory usage bar, disk usage bar |
| **Processes** | Scrollable, sortable table of running processes (PID, name, CPU%, memory MB, status) |
| **About** | Keybinding reference and version info |

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |
| `1` `2` `3` | Switch to tab |
| `←` `→` / `h` `l` | Previous / next tab |
| `↑` `↓` / `j` `k` | Navigate process list |
| `r` | Manual refresh |
| `f` | Cycle refresh interval (1s → 2s → 5s) |
| `s` | Cycle process sort (CPU → Memory → Name → PID) |
| `?` | Toggle help bar |

## Configuration

Create `$XDG_CONFIG_HOME/{{ project-name }}/config.toml`:

```toml
# Configuration is optional — defaults work out of the box.
```

## Demo Mode

```bash
{{ project-name }}-tui --demo
```

Uses simulated data. Useful in containers or restricted environments where `sysinfo` cannot access system statistics.

## Architecture

```
main.rs  →  app.rs (event loop)  →  monitor.rs (sysinfo)
               ↓
            ui.rs (ratatui rendering)
               ↓
           theme.rs (color constants)
```
