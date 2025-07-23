.PHONY: help build run test clean fmt lint check release

# Default target
help:
	@echo "Rumiland CRM - Development Commands"
	@echo ""
	@echo "Usage:"
	@echo "  make build      Build the project"
	@echo "  make run        Run the application"
	@echo "  make test       Run tests"
	@echo "  make clean      Clean build artifacts"
	@echo "  make fmt        Format code"
	@echo "  make lint       Run clippy linter"
	@echo "  make check      Run all checks (fmt, lint, test)"
	@echo "  make release    Build release version"
	@echo "  make admin      Create admin user"

# Build the project
build:
	cargo build

# Run the application
run:
	cargo run

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean
	rm -f rumiland.db

# Format code
fmt:
	cargo fmt

# Run linter
lint:
	cargo clippy -- -D warnings

# Run all checks
check: fmt lint test

# Build release version
release:
	cargo build --release

# Create admin user
admin:
	cargo run create-admin

# Run with environment variables
run-prod:
	DATABASE_URL=sqlite:prod.db \
	SERVER_PORT=8080 \
	RUST_LOG=info \
	cargo run --release
