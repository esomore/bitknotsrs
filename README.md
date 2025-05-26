# BitKnotsRS (bkrs)

A **knots-inspired** Bitcoin node implementation written in Rust.

## 🚀 Features

### Core Infrastructure
- **Actor-based Architecture** - Built with Actix for modular, concurrent processing
- **RocksDB Storage** - High-performance persistent storage with configurable compression
- **Structured Logging** - JSON logging with OpenTelemetry integration
- **Prometheus Metrics** - Comprehensive Bitcoin node metrics
- **Multi-format Events** - ZMQ, Kubernetes Events, and Webhook publishing

### Bitcoin Protocol
- **JSON-RPC API** - Bitcoin Core compatible RPC interface
- **REST API** - Modern HTTP API for blockchain data
- **P2P Networking** - Bitcoin protocol implementation (planned)
- **Transaction Pool** - Mempool management with fee estimation
- **Block Validation** - Full block and transaction validation (planned)

### Cloud-Native Features
- **Kubernetes Integration** - Native K8s event publishing
- **Configuration Management** - TOML-based configuration with CLI overrides
- **Health Checks** - Built-in health endpoints
- **Graceful Shutdown** - Proper resource cleanup

### Development & Testing
- **Comprehensive Test Suite** - 18+ unit tests covering all core components
- **Development Tooling** - 50+ Just commands for streamlined workflow
- **API Testing** - Built-in commands for testing REST and RPC endpoints
- **Database Management** - Tools for initialization, backup, and reset
- **Code Quality** - Automated formatting, linting, and coverage reporting

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   REST API      │    │   JSON-RPC      │    │   Metrics       │
│   :8332         │    │   :18443        │    │   :9090         │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌─────────────────┐
                    │  Event Manager  │
                    │  ZMQ │ K8s │ WH │
                    └─────────────────┘
                                 │
                    ┌─────────────────┐
                    │  Actor System   │
                    └─────────────────┘
                                 │
         ┌───────────┬─────────────────┬───────────┬─────────────┐
         │           │                 │           │             │
    ┌─────────┐ ┌─────────┐ ┌─────────────┐ ┌─────────┐ ┌─────────────┐
    │ Storage │ │ Network │ │   Mempool   │ │  Chain  │ │   Metrics   │
    │  Actor  │ │  Actor  │ │    Actor    │ │  Actor  │ │   Actor     │
    └─────────┘ └─────────┘ └─────────────┘ └─────────┘ └─────────────┘
         │
    ┌─────────┐
    │ RocksDB │
    └─────────┘
```

## 🛠️ Installation

### Prerequisites
- Rust 1.70+
- RocksDB development libraries

### Build from Source
```bash
git clone https://github.com/your-org/bitknotsrs.git
cd bitknotsrs
cargo build --release
```

### Generate Default Configuration
```bash
./target/release/bitknotsrs --generate-config
```

## 🚀 Usage

### Basic Usage
```bash
# Start with default configuration
./target/release/bitknotsrs

# Start with custom config
./target/release/bitknotsrs --config custom.toml

# Override network
./target/release/bitknotsrs --network mainnet

# Override data directory
./target/release/bitknotsrs --datadir /var/lib/bitcoin
```

### Docker
```bash
# Build image
docker build -t bitknotsrs .

# Run container
docker run -p 8332:8332 -p 18443:18443 -p 9090:9090 \
  -v $(pwd)/data:/app/data \
  bitknotsrs
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: bitknotsrs
spec:
  replicas: 1
  selector:
    matchLabels:
      app: bitknotsrs
  template:
    metadata:
      labels:
        app: bitknotsrs
    spec:
      containers:
      - name: bitknotsrs
        image: bitknotsrs:latest
        ports:
        - containerPort: 8332
        - containerPort: 18443
        - containerPort: 9090
        env:
        - name: RUST_LOG
          value: "info"
        volumeMounts:
        - name: data
          mountPath: /app/data
      volumes:
      - name: data
        persistentVolumeClaim:
          claimName: bitknotsrs-data
```

## 📊 Monitoring & Observability

### Prometheus Metrics
Available at `http://localhost:9090/metrics`:

- `bitcoin_chain_height` - Current blockchain height
- `bitcoin_blocks_processed_total` - Total blocks processed
- `bitcoin_transactions_processed_total` - Total transactions processed
- `bitcoin_peers_connected` - Number of connected peers
- `bitcoin_mempool_size` - Current mempool size
- `bitcoin_storage_size_bytes` - Storage size in bytes

### Structured Logging
```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "target": "bitknotsrs::actors::storage",
  "message": "Stored block: 000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
  "block_hash": "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
  "height": 0,
  "event_type": "block"
}
```

### Event Streaming

#### ZMQ Events
```bash
# Subscribe to block events
zmq_sub tcp://localhost:28333 block

# Subscribe to transaction events
zmq_sub tcp://localhost:28333 transaction
```

#### Kubernetes Events
```bash
# View Bitcoin node events
kubectl get events --field-selector involvedObject.name=bitknotsrs-node
```

## 🔧 Configuration

### Network Configuration
```toml
[network_config]
listen_port = 18444
max_peers = 8
connection_timeout_secs = 30

[network_config.zmq]
enabled = true
pub_port = 28332
topics = ["hashblock", "hashtx", "rawblock", "rawtx"]
```

### Storage Configuration
```toml
[storage]
rocks_db_path = "./data/rocksdb"
cache_size = 268435456  # 256MB
compression = "lz4"
backup_enabled = true
backup_interval_hours = 24
```

### Event Publishing
```toml
[events]
enabled_publishers = ["zmq", "k8s", "webhook"]

[events.k8s]
enabled = true
namespace = "bitcoin"
node_name = "bitknotsrs-node"
event_types = ["block", "transaction", "peer", "chain"]

[events.webhook]
enabled = true
endpoints = ["https://your-webhook.com/bitcoin-events"]
timeout_secs = 10
retry_attempts = 3
```

## 🔌 API Reference

### REST API
- `GET /health` - Health check
- `GET /api/v1/info` - Node information
- `GET /api/v1/stats` - Node statistics
- `GET /api/v1/peers` - Connected peers
- `GET /api/v1/mempool` - Mempool information
- `GET /api/v1/block?hash=<hash>` - Get block by hash
- `GET /api/v1/transaction?txid=<txid>` - Get transaction
- `POST /api/v1/sendrawtransaction` - Broadcast transaction

### JSON-RPC API
Compatible with Bitcoin Core RPC:
- `getblockchaininfo`
- `getbestblockhash`
- `getblock <hash>`
- `getblockhash <height>`
- `getrawtransaction <txid>`
- `sendrawtransaction <hex>`
- `getmempoolinfo`
- `getpeerinfo`

## 🧪 Development

### Prerequisites
- [Just](https://github.com/casey/just) command runner (recommended)
- [Radicle](https://radicle.xyz) for decentralized issue management
- Rust 1.70+
- RocksDB development libraries

### Quick Start with Just
```bash
# List all available commands
just --list

# Run all tests
just test

# Quick development check (format, lint, test)
just quick-check

# Development workflow with Radicle sync
just dev-workflow

# Build and run in regtest mode
just build
just run-regtest
```

### 🌐 Radicle Integration

This project uses [Radicle](https://radicle.xyz) for decentralized issue management and collaboration:

```bash
# Issue Management
just issues                    # List all issues
just issue <id>               # View specific issue
just draft-issue <name>       # Create issue draft locally
just create-issue "<title>" <draft>  # Create Radicle issue from draft

# Patch Management
just patches                  # List all patches
just patch <id>              # View specific patch
just patch-prep              # Prepare code for patch submission
just patch-new               # Create new patch

# Collaboration
just sync                    # Sync with Radicle network
just rad-peers              # Show connected Radicle peers
just rad-status             # Show Radicle node status
```

### 📝 Issue Workflow

The project uses a local `.issues/` workspace for drafting before creating Radicle issues:

```
.issues/
├── README.md              # Workflow documentation
├── templates/             # Issue and patch templates
│   ├── issue-template.md  # Standard issue format
│   └── patch-template.md  # Standard patch format
├── drafts/               # Work-in-progress (git-ignored)
└── temp/                 # Temporary files (git-ignored)
```

**Workflow**:
1. **Draft locally**: `just draft-issue networking-feature`
2. **Edit draft**: `.issues/drafts/networking-feature.md`
3. **Create in Radicle**: `just create-issue "Networking Feature" networking-feature`
4. **Track progress**: `just issue <issue-id>`
5. **Clean up**: `just clean-drafts`

This approach gives you local file editing benefits while keeping Radicle as the source of truth.

### Testing
The project includes comprehensive unit tests covering all core components:

```bash
# Run all tests (18 tests)
just test

# Run tests with verbose output
just test-verbose

# Run only unit tests
just test-unit

# Run specific test
just test-one test_storage_initialization

# Run tests with coverage (requires cargo-tarpaulin)
just test-coverage
```

**Test Coverage:**
- ✅ Configuration parsing and validation
- ✅ Error handling and display
- ✅ Storage operations (blocks, transactions, UTXOs, mempool)
- ✅ Actor message serialization
- ✅ Database initialization and statistics

### Development Workflow
```bash
# Format code
just fmt

# Run linter
just lint

# Fix linting issues
just fix

# Watch for changes and run tests
just watch

# Generate project statistics
just stats

# Clean build artifacts and data
just clean
```

### API Testing
```bash
# Test REST API endpoints
just test-api-health
just test-api-info
just test-api-stats

# Test JSON-RPC endpoints
just test-rpc-info
just test-rpc-hash
```

### Database Management
```bash
# Initialize database with test data
just init-db

# Reset database
just reset-db

# Backup database
just backup-db
```

### Traditional Cargo Commands
If you prefer using Cargo directly:

```bash
# Run tests
cargo test

# Development mode
RUST_LOG=debug cargo run

# Run with specific config
cargo run -- --config dev.toml --network regtest
```

### Just Command Reference
Most commonly used commands:

| Command | Description |
|---------|-------------|
| `just test` | Run all tests |
| `just build` | Build in debug mode |
| `just run-regtest` | Run in regtest mode |
| `just quick-check` | Format, lint, and test |
| `just dev` | Run with auto-reload |
| `just clean` | Clean all artifacts |
| `just stats` | Show project statistics |
| `just --list` | Show all available commands |

### 🚀 Current Development Status

**Active Issue**: [Implement P2P Networking and Peer Discovery](https://app.radicle.xyz/nodes/z6MksFqXN3Yhqk8pTJdUGLwATkRfQvwZXPqR2qMEhbS9wzpT/rad:z32DWCTKTNy7dxJdmrBAe2cVR1bfktSMTqMT1hBPWEVk4GNp9M/issues/09a530958b8db3b8899b9531edc4ae4cea041f7b)
*Issue ID: `09a530958b8db3b8899b9531edc4ae4cea041f7b`*

**Current Phase**: Network Foundation
- [ ] Create network constants module with magic bytes, ports, DNS seeds per network
- [ ] Enhance configuration for network-specific settings
- [ ] Generate network-specific config files (mainnet.toml, testnet.toml)
- [ ] Add network-aware Just commands

**Next Phases**: Peer Discovery → Connection Management → Protocol Messages → Integration

### Contributing

This project uses **Radicle** for decentralized collaboration:

**🌐 Radicle Project**: [https://app.radicle.xyz/nodes/seed.radicle.garden/rad:zz8CzpVqLxYXKHsiKzDBPBa474HQ](https://app.radicle.xyz/nodes/seed.radicle.garden/rad:zz8CzpVqLxYXKHsiKzDBPBa474HQ)

1. **Check active issues**: `just issues`
2. **Create feature branch**: `git checkout -b feature/your-feature`
3. **Draft your work**: `just draft-issue your-feature` (optional)
4. **Make your changes** and add tests
5. **Prepare patch**: `just patch-prep`
6. **Submit patch**: `just patch-new`
7. **Sync with network**: `just sync`

For traditional Git workflows, you can still create pull requests, but we encourage using Radicle patches for a truly decentralized experience.

## 📋 Roadmap

### Phase 1: Infrastructure ✅
- [x] Actor system architecture
- [x] RocksDB storage layer
- [x] Configuration management
- [x] Logging and metrics
- [x] Event publishing (ZMQ, K8s, Webhook)
- [x] Comprehensive testing suite (18+ unit tests)
- [x] Development tooling (50+ Just commands)
- [x] API testing infrastructure

### Phase 2: Bitcoin Protocol (In Progress)
- [ ] P2P networking implementation
- [ ] Block and transaction validation
- [ ] UTXO set management
- [ ] Mempool fee estimation
- [ ] Initial block download

### Phase 3: Advanced Features
- [ ] Pruning support
- [ ] Compact block filters
- [ ] Lightning Network integration
- [ ] Advanced monitoring dashboards

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Acknowledgments

- Inspired by Bitcoin Knots
- Built with the Rust Bitcoin ecosystem
- Powered by Actix and RocksDB

---

**BitKnotsRS** - A modern, observable, cloud-native Bitcoin node implementation.