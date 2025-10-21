.PHONY: build test ci clean format lint

# Build the package in development mode
build:
	uv run maturin develop

# Run all tests
test: build
	uv run pytest -s -v

# Code quality checks (CI)
ci: build
	uv run ruff check --fix
	rustfmt --check src/*.rs
	cargo clippy -- -W clippy::pedantic -D warnings

# Format code
format:
	uv run ruff format .
	rustfmt src/*.rs

# Lint code
lint:
	uv run ruff check .
	cargo clippy -- -W clippy::pedantic

# Clean build artifacts
clean:
	uv clean
	cargo clean
	rm -rf target/ dist/ build/ *.egg-info/
	find . -type d -name __pycache__ -delete
	find . -type f -name "*.pyc" -delete

# Build wheel for distribution
build-wheel:
	uv run maturin build --release -i python