# Makefile for rlinks
# Builds the workspace and collects release binaries into dist/

SHELL := /bin/bash
BINARIES := rlinks-cli rlinks-worker rlinks-bh2 bench-harness

.PHONY: all build dist test fmt clippy clean

all: build dist

# Build release artifacts for the entire workspace
build:
	cargo build --workspace --release

# Collect release binaries into ./dist
dist: build
	@mkdir -p dist
	@echo "Copying binaries to dist/"
	@for bin in $(BINARIES); do \
		src=target/release/$$bin; \
		if [ -f $$src ]; then \
			cp $$src dist/; \
			echo "  - $$bin"; \
		else \
			echo "  - skipped $$bin (not built)"; \
		fi; \
	done
	@ls -la dist || true

# Run the test suite for the workspace
test:
	cargo test --workspace

# Formatting and lint helpers
fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

# Clean build artifacts and dist folder
clean:
	cargo clean
	@rm -rf dist
