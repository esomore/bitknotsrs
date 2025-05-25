# BitKnotsRS Development Justfile
# Run `just --list` to see all available commands

# Default recipe - show help
default:
    @just --list

# === BUILD COMMANDS ===

# Build the project in debug mode
build:
    cargo build

# Build the project in release mode
build-release:
    cargo build --release

# Clean and rebuild
rebuild:
    cargo clean
    cargo build

# Check if the project compiles without building
check:
    cargo check

# === RUN COMMANDS ===

# Run with default config
run:
    cargo run

# Run with custom config file
run-config config_file:
    cargo run -- --config {{config_file}}

# Run in regtest mode
run-regtest:
    cargo run -- --network regtest

# Run in testnet mode
run-testnet:
    cargo run -- --network testnet

# Run with custom data directory
run-datadir datadir:
    cargo run -- --datadir {{datadir}}

# Generate default config file
generate-config:
    cargo run -- --generate-config

# Generate test config file
generate-test-config:
    cargo run -- --config test-config.toml --generate-config

# === TEST COMMANDS ===

# Run all tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run only unit tests
test-unit:
    cargo test --lib

# Run tests with coverage (requires cargo-tarpaulin)
test-coverage:
    cargo tarpaulin --out Html

# Run specific test
test-one test_name:
    cargo test {{test_name}}

# === DEVELOPMENT COMMANDS ===

# Run in development mode with auto-reload (requires cargo-watch)
dev:
    cargo watch -x run

# Watch for changes and run tests
watch:
    cargo watch -x test

# Format code
fmt:
    cargo fmt

# Run clippy linter
lint:
    cargo clippy

# Fix linting issues automatically
fix:
    cargo fix --allow-dirty --allow-staged

# Quick check (format, lint, test)
quick-check:
    cargo fmt
    cargo clippy
    cargo test

# === DOCKER COMMANDS ===

# Build Docker image
docker-build:
    docker build -t bitknotsrs .

# Run in Docker container
docker-run:
    docker run -p 8332:8332 -p 18443:18443 -v $(pwd)/data:/app/data bitknotsrs

# Stop Docker container
docker-stop:
    docker stop $(docker ps -q --filter ancestor=bitknotsrs)

# === API/RPC TESTING ===

# Test API health endpoint
test-api-health:
    curl -s http://localhost:8332/health | jq .

# Test API node info
test-api-info:
    curl -s http://localhost:8332/api/v1/info | jq .

# Test API node stats
test-api-stats:
    curl -s http://localhost:8332/api/v1/stats | jq .

# Test API peers
test-api-peers:
    curl -s http://localhost:8332/api/v1/peers | jq .

# Test API mempool
test-api-mempool:
    curl -s http://localhost:8332/api/v1/mempool | jq .

# Test RPC getblockchaininfo
test-rpc-info:
    curl -s -X POST http://localhost:18443 \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"getblockchaininfo","params":[],"id":1}' | jq .

# Test RPC getbestblockhash
test-rpc-hash:
    curl -s -X POST http://localhost:18443 \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"getbestblockhash","params":[],"id":1}' | jq .

# === UTILITY COMMANDS ===

# Clean build artifacts and data
clean:
    cargo clean
    rm -rf data/
    rm -rf test_data/
    rm -rf logs/

# Clean only data directories
clean-data:
    rm -rf data/
    rm -rf test_data/

# Backup data directory
backup:
    tar -czf backup-$(date +%Y%m%d-%H%M%S).tar.gz data/

# Show project statistics
stats:
    @echo "=== Project Statistics ==="
    @echo "Lines of code:"
    @find src -name "*.rs" -exec wc -l {} + | tail -1
    @echo ""
    @echo "Test count:"
    @grep -r "#\[test\]" src --include="*.rs" | wc -l
    @echo ""
    @echo "Dependencies:"
    @grep "^[a-zA-Z]" Cargo.toml | grep "=" | wc -l

# Show system requirements
requirements:
    @echo "=== System Requirements ==="
    @echo "Rust version: $(rustc --version)"
    @echo "Cargo version: $(cargo --version)"
    @echo "Available disk space:"
    @df -h . | tail -1
    @echo "Available memory:"
    @free -h 2>/dev/null || vm_stat | head -5

# === BENCHMARKING ===

# Run benchmarks (requires nightly Rust)
bench:
    cargo +nightly bench

# Profile the application (requires cargo-flamegraph)
profile:
    cargo flamegraph --bin bitknotsrs

# === DATABASE COMMANDS ===

# Initialize database with test data
init-db:
    cargo run -- --config test-config.toml --generate-config
    cargo run -- --config test-config.toml

# Reset database
reset-db:
    rm -rf data/rocksdb/
    rm -rf test_data/rocksdb/

# Backup database
backup-db:
    tar -czf db-backup-$(date +%Y%m%d-%H%M%S).tar.gz data/rocksdb/

# === MONITORING ===

# Show metrics (if metrics server is running)
metrics:
    curl -s http://localhost:9090/metrics

# Show logs (if file logging is enabled)
logs:
    tail -f logs/bitknotsrs.log

# Monitor system resources
monitor:
    watch -n 1 'ps aux | grep bitknotsrs; echo ""; df -h .; echo ""; free -h'

# === RELEASE COMMANDS ===

# Prepare for release (format, lint, test, build)
release-prep:
    cargo fmt
    cargo clippy -- -D warnings
    cargo test
    cargo build --release

# Create release build with optimizations
release-build:
    RUSTFLAGS="-C target-cpu=native" cargo build --release

# === HELP COMMANDS ===

# Show build help
help-build:
    @echo "=== Build Commands ==="
    @echo "build          - Build in debug mode"
    @echo "build-release  - Build in release mode"
    @echo "rebuild        - Clean and rebuild"
    @echo "check          - Check compilation without building"

# Show run help
help-run:
    @echo "=== Run Commands ==="
    @echo "run            - Run with default config"
    @echo "run-config     - Run with custom config file"
    @echo "run-regtest    - Run in regtest mode"
    @echo "run-testnet    - Run in testnet mode"
    @echo "run-datadir    - Run with custom data directory"

# Show test help
help-test:
    @echo "=== Test Commands ==="
    @echo "test           - Run all tests"
    @echo "test-verbose   - Run tests with output"
    @echo "test-unit      - Run only unit tests"
    @echo "test-coverage  - Run tests with coverage"
    @echo "test-one       - Run specific test"

# Show development help
help-dev:
    @echo "=== Development Commands ==="
    @echo "dev            - Run with auto-reload"
    @echo "watch          - Watch and run tests"
    @echo "fmt            - Format code"
    @echo "lint           - Run clippy linter"
    @echo "fix            - Fix linting issues"
    @echo "quick-check    - Format, lint, and test"

# Show all help sections
help-all:
    @just help-build
    @echo ""
    @just help-run
    @echo ""
    @just help-test
    @echo ""
    @just help-dev
