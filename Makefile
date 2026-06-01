.DEFAULT_GOAL := help

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-22s\033[0m %s\n", $$1, $$2}'

build: ## Compile the project
	@cargo build

check: ## Fast compile check (no codegen)
	@cargo check --workspace --all-targets --all-features

run: ## Launch the TUI
	@cargo run

test: ## Run tests with nextest
	@cargo nextest run --all-features

fmt: ## Check code formatting with nightly rustfmt
	@cargo +nightly fmt -- --check

clippy: ## Lint with pedantic clippy rules
	@cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic

lint: fmt clippy ## Run fmt and clippy

audit: ## Run security audit (deps + supply chain)
	@cargo deny check
	@cargo audit
	@cargo vet check 2>/dev/null || echo "cargo-vet not configured; run 'cargo vet init' to set up"

install-tools: ## Install development toolchain
	@pip install pre-commit 2>/dev/null || echo "Install pre-commit manually"
	@cargo install cargo-deny --locked 2>/dev/null || true
	@cargo install cargo-audit --locked 2>/dev/null || true
	@cargo install cargo-nextest --locked 2>/dev/null || true
	@cargo install cargo-vet --locked 2>/dev/null || true
	@cargo install typos-cli 2>/dev/null || true
	@cargo install cargo-release --locked 2>/dev/null || true
	@which gitleaks >/dev/null 2>&1 || echo "Install gitleaks: https://github.com/gitleaks/gitleaks#installing"
	@pre-commit install
	@echo "Run 'pre-commit run --all-files' to verify."

install: build ## Install the TUI binary locally
	@cargo install --path apps/tui

demo: build ## Launch the TUI with simulated data
	@cargo run -- --demo

watch: ## Watch for changes and check (requires cargo-watch)
	@cargo watch -x check

bench: ## Run benchmarks
	@cargo bench --workspace

coverage: ## Generate test coverage report
	@cargo llvm-cov --html --open

docs: ## Generate and open API documentation
	@cargo doc --no-deps --open

release-dry-run: ## Preview release without executing
	@cargo release --dry-run

update-submodule: ## Update git submodules recursively
	@git submodule update --init --recursive --remote

check-agent-sync: ## Verify CLAUDE.md exists
	@test -f CLAUDE.md || { \
		echo "CLAUDE.md is required for project-level agent instructions."; \
		exit 1; \
	}

release: ## Tag and publish a release
	@cargo release tag --execute
	@git cliff -o CHANGELOG.md
	@git commit -a -n -m "chore: update CHANGELOG.md" || true
	@git push origin master
	@cargo release push --execute

.PHONY: help build check run demo test fmt clippy lint audit install-tools install \
        watch bench coverage docs release-dry-run \
        update-submodule check-agent-sync release
