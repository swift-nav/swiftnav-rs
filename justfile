set dotenv-load

# What is run by default when no command is given
default: fmt lint test

# Build
build:
    @cargo build --workspace --all-features

# Run tests
test:
    @cargo test --workspace --all-features

# Run cargo check
check:
    @cargo check --workspace --all-features

# Format code
fmt:
    @cargo fmt --all

# Check formatting without making changes
fmt-check:
    @cargo fmt --all --check

# Lint code
lint:
    @cargo clippy --workspace --all-features

# Generate docs
docs:
    @cargo doc  --workspace --all-features --no-deps

# Generate and open docs
open-docs:
    @cargo doc  --workspace --all-features --no-deps --open