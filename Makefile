.PHONY: help setup install dev test test-coverage lint lint-fix build build-release clean
.PHONY: audit audit-rust audit-js audit-docker audit-secrets
.PHONY: pre-commit ci docker-up docker-down docker-logs
.PHONY: format check security

# Default target
help:
	@echo "ObsidianBountyFinder - Development Commands"
	@echo ""
	@echo "Setup:"
	@echo "  make setup          - Install dependencies and hooks"
	@echo "  make install        - Install Rust and npm dependencies"
	@echo ""
	@echo "Development:"
	@echo "  make dev            - Start development server"
	@echo "  make test           - Run all tests"
	@echo "  make test-coverage  - Run tests with coverage"
	@echo ""
	@echo "Code Quality:"
	@echo "  make lint           - Run all linters"
	@echo "  make lint-fix       - Fix linting issues"
	@echo "  make format         - Format code"
	@echo "  make check          - Run all checks"
	@echo ""
	@echo "Security:"
	@echo "  make audit          - Run all security audits"
	@echo "  make audit-rust     - Run Rust security audit"
	@echo "  make audit-js       - Run JS security audit"
	@echo "  make audit-docker   - Run Docker security audit"
	@echo "  make audit-secrets  - Run secrets detection"
	@echo ""
	@echo "Build:"
	@echo "  make build          - Build project"
	@echo "  make build-release  - Build release version"
	@echo "  make clean          - Clean build artifacts"
	@echo ""
	@echo "Docker:"
	@echo "  make docker-up      - Start Docker containers"
	@echo "  make docker-down    - Stop Docker containers"
	@echo "  make docker-logs    - View Docker logs"
	@echo ""
	@echo "CI/CD:"
	@echo "  make pre-commit     - Run pre-commit hooks"
	@echo "  make ci             - Run CI locally with act"

# Setup
setup: install pre-commit
	@echo "Setup complete!"

install:
	@echo "Installing dependencies..."
	@# Rust dependencies are fetched on build
	@# Install pre-commit hooks
	@if command -v pip3 >/dev/null 2>&1; then \
		pip3 install pre-commit; \
	fi
	@echo "Dependencies installed!"

# Development
dev:
	@echo "Starting development server..."
	@cargo run --bin obsidian-bounty-finder

test:
	@echo "Running tests..."
	@cargo test --workspace --all-features
	@cd frontend && npm run test

test-coverage:
	@echo "Running tests with coverage..."
	@cargo tarpaulin --out Xml --coveralls-xml --workspace
	@cd frontend && npm run test -- --coverage

# Code Quality
lint:
	@echo "Running linters..."
	@cargo fmt -- --check
	@cargo clippy -- -D warnings
	@cd frontend && npm run lint
	@cd frontend && npm run typecheck

lint-fix:
	@echo "Fixing linting issues..."
	@cargo fmt
	@cd frontend && npm run lint:fix

format:
	@echo "Formatting code..."
	@cargo fmt
	@cd frontend && npm run format

check: lint test

# Security
audit:
	@echo "Running all security audits..."
	@make audit-rust
	@make audit-js
	@make audit-docker
	@make audit-secrets

audit-rust:
	@echo "Running Rust security audit..."
	@cargo audit || true
	@cargo deny check || true

audit-js:
	@echo "Running JS security audit..."
	@cd frontend && npm audit --audit-level=moderate || true

audit-docker:
	@echo "Running Docker security audit..."
	@if command -v trivy >/dev/null 2>&1; then \
		trivy fs --severity HIGH,CRITICAL . || true; \
	fi
	@hadolint Dockerfile || true

audit-secrets:
	@echo "Running secrets detection..."
	@if command -v trufflehog >/dev/null 2>&1; then \
		trufflehog filesystem . --json > trufflehog.json || true; \
	fi

security: audit

# Build
build:
	@echo "Building project..."
	@cargo build --workspace
	@cd frontend && npm run build

build-release:
	@echo "Building release version..."
	@cargo build --release --workspace
	@cd frontend && npm run build

clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@cd frontend && npm run clean

# Docker
docker-up:
	@echo "Starting Docker containers..."
	@docker-compose up -d

docker-down:
	@echo "Stopping Docker containers..."
	@docker-compose down

docker-logs:
	@docker-compose logs -f

# CI/CD
pre-commit:
	@echo "Running pre-commit hooks..."
	@if command -v pre-commit >/dev/null 2>&1; then \
		pre-commit run --all-files; \
	else \
		echo "pre-commit not installed. Run 'make setup'"; \
	fi

ci:
	@echo "Running CI locally with act..."
	@if command -v act >/dev/null 2>&1; then \
		act pull_request; \
	else \
		echo "act not installed. Install from: https://github.com/nektos/act"; \
	fi
