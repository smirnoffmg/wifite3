.PHONY: help run test lint build clean

help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

run: ## Run the CLI with scanning
	uv run python wifite3/cli.py --scan -i en0 -v

test: ## Run tests
	uv run pytest tests/ -v

test-sudo: ## Run tests with sudo (for network scanning tests)
	sudo uv run pytest tests/ -v

lint: ## Run linters
	uv run ruff check .
	uv run ruff format .

build: ## Build the Rust module
	uv run maturin develop

clean: ## Clean build artifacts
	cargo clean
	rm -rf target/
	rm -rf .venv/

install: ## Install dependencies
	uv sync
