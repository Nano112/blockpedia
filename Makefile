# Blockpedia Build Tools
# =====================

.PHONY: help data clean build test

help:  ## Show this help message
	@echo "Blockpedia Build Tools"
	@echo "======================"
	@echo ""
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'

data:  ## Download and cache block data for offline builds
	@echo "Downloading block data sources..."
	cargo run --bin build-data --features build-data
	@echo "Data files saved to ./data/"
	@echo "You can now build without network access using: cargo build --features use-prebuilt"

clean:  ## Clean cached data and build artifacts
	rm -rf data/
	cargo clean

build:  ## Build blockpedia with default features
	cargo build --release

build-prebuilt:  ## Build blockpedia using pre-built data (no network required)
	cargo build --release --features use-prebuilt

test:  ## Run tests
	cargo test

test-prebuilt:  ## Run tests using pre-built data
	cargo test --features use-prebuilt

check:  ## Check code without building
	cargo check

fmt:  ## Format code
	cargo fmt

clippy:  ## Run clippy lints
	cargo clippy

doc:  ## Generate documentation
	cargo doc --open

# Development helpers
dev-setup: data  ## Set up development environment
	@echo "Development environment ready!"
	@echo "Try: make build-prebuilt"
