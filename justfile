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

# Run in mainnet mode
run-mainnet:
    cargo run -- --network mainnet

# Run with mainnet config file
run-mainnet-config:
    cargo run -- --config config/mainnet.toml

# Run with testnet config file
run-testnet-config:
    cargo run -- --config config/testnet.toml

# Run with regtest config file
run-regtest-config:
    cargo run -- --config config/regtest.toml

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

# Development workflow with Radicle sync
dev-workflow:
    cargo fmt
    cargo clippy
    cargo test
    rad sync

# Prepare patch (format, lint, test, sync)
patch-prep:
    cargo fmt
    cargo clippy -- -D warnings
    cargo test
    rad sync
    @echo "Ready to create patch with: just patch-new"

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

# === NETWORK-SPECIFIC API TESTING ===

# Test mainnet API (port 8332)
test-api-mainnet:
    curl -s http://localhost:8332/health | jq .
    curl -s http://localhost:8332/api/v1/info | jq .

# Test testnet API (port 18332)
test-api-testnet:
    curl -s http://localhost:18332/health | jq .
    curl -s http://localhost:18332/api/v1/info | jq .

# Test regtest API (port 8332)
test-api-regtest:
    curl -s http://localhost:8332/health | jq .
    curl -s http://localhost:8332/api/v1/info | jq .

# Test mainnet RPC (port 8333)
test-rpc-mainnet:
    curl -s -X POST http://localhost:8333 \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"getblockchaininfo","params":[],"id":1}' | jq .

# Test testnet RPC (port 18333)
test-rpc-testnet:
    curl -s -X POST http://localhost:18333 \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"getblockchaininfo","params":[],"id":1}' | jq .

# Test regtest RPC (port 18443)
test-rpc-regtest:
    curl -s -X POST http://localhost:18443 \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"getblockchaininfo","params":[],"id":1}' | jq .

# === RADICLE COMMANDS ===

# List all issues
issues:
    rad issue list

# Show specific issue
issue id:
    rad issue show {{id}}

# Create new issue
issue-new:
    rad issue open

# List all patches
patches:
    rad patch list

# Show specific patch
patch id:
    rad patch show {{id}}

# Create new patch from current branch
patch-new:
    rad patch open

# Sync with Radicle network
sync:
    rad sync

# Show repository information
rad-info:
    rad inspect

# Show connected peers
rad-peers:
    rad node sessions

# Clone a Radicle repository
rad-clone repo_id:
    rad clone {{repo_id}}

# Initialize current repository for Radicle
rad-init:
    rad init

# Publish repository to Radicle network
rad-publish:
    rad push

# Show Radicle node status
rad-status:
    rad node status

# Start Radicle node
rad-start:
    rad node start

# Stop Radicle node
rad-stop:
    rad node stop

# Show Radicle identity
rad-id:
    rad self

# Search issues by keyword (case-insensitive)
search-issues keyword:
    @echo "=== Searching issues for: {{keyword}} ==="
    rad issue list | grep -i "{{keyword}}" || echo "No issues found matching '{{keyword}}'"

# Show issue summary (titles only)
issues-summary:
    @echo "=== All Issues Summary ==="
    rad issue list | head -20

# === ISSUE WORKSPACE COMMANDS ===

# Create new issue draft from template
draft-issue name:
    cp .issues/templates/issue-template.md .issues/drafts/{{name}}.md
    @echo "Created issue draft: .issues/drafts/{{name}}.md"

# Create new patch draft from template
draft-patch name:
    cp .issues/templates/patch-template.md .issues/drafts/{{name}}.md
    @echo "Created patch draft: .issues/drafts/{{name}}.md"

# Create issue from draft
create-issue title draft:
    rad issue open --title "{{title}}" --description "$(cat .issues/drafts/{{draft}}.md)"

# Create patch from draft
create-patch title draft:
    rad patch open --title "{{title}}" --description "$(cat .issues/drafts/{{draft}}.md)"

# Clean up temporary files
clean-drafts:
    rm -f .issues/drafts/*.md
    rm -f .issues/temp/*.md

# List current drafts
list-drafts:
    @echo "=== Issue/Patch Drafts ==="
    @ls -la .issues/drafts/ 2>/dev/null || echo "No drafts found"

# Show issue creation checklist
issue-checklist:
    @cat .issues/templates/issue-checklist.md

# === GIT WORKFLOW COMMANDS ===

# Check git status and show ignored files
git-status:
    git status
    @echo ""
    @echo "=== Recently ignored files ==="
    @git ls-files --others --ignored --exclude-standard | head -10

# Clean git ignored files
git-clean:
    git clean -fdX

# Show what would be cleaned
git-clean-dry:
    git clean -fdXn

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

# Initialize mainnet database
init-db-mainnet:
    cargo run -- --config config/mainnet.toml --generate-config
    mkdir -p data/mainnet logs/mainnet
    cargo run -- --config config/mainnet.toml

# Initialize testnet database
init-db-testnet:
    cargo run -- --config config/testnet.toml --generate-config
    mkdir -p data/testnet logs/testnet
    cargo run -- --config config/testnet.toml

# Initialize regtest database
init-db-regtest:
    cargo run -- --config config/regtest.toml --generate-config
    mkdir -p data/regtest logs/regtest
    cargo run -- --config config/regtest.toml

# Reset database
reset-db:
    rm -rf data/rocksdb/
    rm -rf test_data/rocksdb/

# Reset all network databases
reset-db-all:
    rm -rf data/mainnet/rocksdb/
    rm -rf data/testnet/rocksdb/
    rm -rf data/regtest/rocksdb/
    rm -rf test_data/rocksdb/

# Reset mainnet database
reset-db-mainnet:
    rm -rf data/mainnet/rocksdb/

# Reset testnet database
reset-db-testnet:
    rm -rf data/testnet/rocksdb/

# Reset regtest database
reset-db-regtest:
    rm -rf data/regtest/rocksdb/

# Backup database
backup-db:
    tar -czf db-backup-$(date +%Y%m%d-%H%M%S).tar.gz data/rocksdb/

# Backup all network databases
backup-db-all:
    tar -czf db-backup-all-$(date +%Y%m%d-%H%M%S).tar.gz data/

# Backup mainnet database
backup-db-mainnet:
    tar -czf db-backup-mainnet-$(date +%Y%m%d-%H%M%S).tar.gz data/mainnet/

# Backup testnet database
backup-db-testnet:
    tar -czf db-backup-testnet-$(date +%Y%m%d-%H%M%S).tar.gz data/testnet/

# Backup regtest database
backup-db-regtest:
    tar -czf db-backup-regtest-$(date +%Y%m%d-%H%M%S).tar.gz data/regtest/

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
    @echo "run                 - Run with default config"
    @echo "run-config          - Run with custom config file"
    @echo "run-regtest         - Run in regtest mode"
    @echo "run-testnet         - Run in testnet mode"
    @echo "run-mainnet         - Run in mainnet mode"
    @echo "run-regtest-config  - Run with regtest config file"
    @echo "run-testnet-config  - Run with testnet config file"
    @echo "run-mainnet-config  - Run with mainnet config file"
    @echo "run-datadir         - Run with custom data directory"

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

# Show Radicle help
help-rad:
    @echo "=== Radicle Commands ==="
    @echo "issues         - List all issues"
    @echo "issues-summary - Show issue titles only"
    @echo "search-issues <keyword> - Search issues by keyword"
    @echo "issue <id>     - Show specific issue"
    @echo "issue-new      - Create new issue"
    @echo "patches        - List all patches"
    @echo "patch <id>     - Show specific patch"
    @echo "patch-new      - Create new patch from current branch"
    @echo "sync           - Sync with Radicle network"
    @echo "rad-info       - Show repository information"
    @echo "rad-peers      - Show connected peers"
    @echo "rad-status     - Show Radicle node status"
    @echo "rad-start      - Start Radicle node"
    @echo "rad-stop       - Stop Radicle node"
    @echo "rad-id         - Show Radicle identity"
    @echo ""
    @echo "=== Issue Workspace Commands ==="
    @echo "issue-checklist        - Show issue creation checklist"
    @echo "draft-issue <name>     - Create issue draft from template"
    @echo "draft-patch <name>     - Create patch draft from template"
    @echo "create-issue <title> <draft> - Create Radicle issue from draft"
    @echo "create-patch <title> <draft> - Create Radicle patch from draft"
    @echo "list-drafts            - List current drafts"
    @echo "clean-drafts           - Clean up temporary files"

# Show all help sections
help-all:
    @just help-build
    @echo ""
    @just help-run
    @echo ""
    @just help-test
    @echo ""
    @just help-dev
    @echo ""
    @just help-rad
