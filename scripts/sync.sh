#!/bin/bash
set -euo pipefail

SOURCE="${1:?Usage: sync.sh <path-to-other-template>}"
MODE="${2:-check}"

shared_files=(
    "crates/core/Cargo.toml"
    "crates/core/src/lib.rs"
    "crates/core/benches/config_bench.rs"
    "crates/core/examples/config_demo.rs"
    "crates/core/tests/integration_test.rs"
    "rust-toolchain.toml"
    "rustfmt.toml"
    ".cargo/config.toml"
    "deny.toml"
    ".pre-commit-config.yaml"
    "_typos.toml"
    ".tokeignore"
    ".gitignore"
    ".env.example"
    "LICENSE.md"
    "SECURITY.md"
    "CONTRIBUTING.md"
    "CODE_OF_CONDUCT.md"
    "docs/pre-commit-usage.md"
    "docs/release.md"
)

changed=0
for f in "${shared_files[@]}"; do
    if [ ! -f "$SOURCE/$f" ]; then
        echo "MISSING: $SOURCE/$f"
        continue
    fi
    if ! diff -q "$SOURCE/$f" "$f" > /dev/null 2>&1; then
        changed=$((changed + 1))
        echo "DIFF: $f"
        diff "$SOURCE/$f" "$f" || true
        if [ "$MODE" = "apply" ]; then
            cp "$SOURCE/$f" "$f"
            echo "  -> synced"
        fi
    fi
done

if [ "$changed" -eq 0 ]; then
    echo "All ${#shared_files[@]} files in sync."
else
    echo "$changed files out of sync. Run with 'apply' to sync."
fi
