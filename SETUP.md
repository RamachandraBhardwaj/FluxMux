# FluxMux - Setup and Installation Guide

## ✅ Application Status: WORKING

Your FluxMux application is now fully functional and ready to use!

## What Was Fixed

1. **Rust Edition**: Changed all `edition = "2024"` to `edition = "2021"` in all Cargo.toml files
   - The 2024 edition is not yet stable in Rust
   - Fixed in: root, fluxmux-connectors, fluxmux-sinks, fluxmux-plugins, fluxmux-ui, and example crates

2. **Rust Installation**: Installed Rust toolchain (version 1.91.0)

3. **Dependencies**: All dependencies compiled successfully including:
   - Kafka support (rdkafka)
   - PostgreSQL support (tokio-postgres)
   - Format support (JSON, YAML, TOML, CSV, Parquet, Avro, MsgPack, CBOR)

## Build Instructions

### Development Build
```bash
# Build all workspace members
cargo build --workspace

# Build just the CLI
cargo build -p fluxmux-cli
```

### Release Build (Optimized)
```bash
# Build optimized release version
cargo build --release -p fluxmux-cli

# The binary will be at: target/release/fluxmux-cli
```

## Installation

### Option 1: Use from build directory
```bash
# Run directly from target directory
./target/release/fluxmux-cli --help
```

### Option 2: Install to PATH
```bash
# Install to ~/.cargo/bin (automatically in PATH)
cargo install --path crates/fluxmux-cli

# Now you can run from anywhere
fluxmux-cli --help
```

### Option 3: Create alias
Add to your `~/.zshrc`:
```bash
alias fluxmux='/Users/anuragnarsingoju/Academics/FluxMux/target/release/fluxmux-cli'
```

Then reload:
```bash
source ~/.zshrc
```

## Quick Test

### Test 1: Convert JSON to YAML
```bash
cargo run -p fluxmux-cli -- convert input.json output.yaml --from json --to yaml
```

### Test 2: Convert JSON to CSV
```bash
cargo run -p fluxmux-cli -- convert input.json output.csv --from json --to csv
```

### Test 3: Help for all commands
```bash
# Main help
cargo run -p fluxmux-cli -- --help

# Convert help
cargo run -p fluxmux-cli -- convert --help

# Bridge help
cargo run -p fluxmux-cli -- bridge --help

# Pipe help
cargo run -p fluxmux-cli -- pipe --help

# Kafka help
cargo run -p fluxmux-cli -- kafka --help
```

## Usage Examples

### 1. Convert Command
```bash
# JSON to YAML
fluxmux-cli convert input.json output.yaml --from json --to yaml

# CSV to JSON
fluxmux-cli convert data.csv data.json --from csv --to json

# YAML to TOML
fluxmux-cli convert config.yaml config.toml --from yaml --to toml
```

### 2. Bridge Command
```bash
# File to Kafka with batching
fluxmux-cli bridge \
    --source file:input.json \
    --sink kafka://localhost:9092/topic \
    --batch-size 10 \
    --retry-max-attempts 3

# Kafka to PostgreSQL
fluxmux-cli bridge \
    --source kafka://localhost:9092/input-topic?group=my-group \
    --sink postgres://localhost:5432/mydb?table=events \
    --deduplicate \
    --throttle-per-sec 100

# stdin to stdout with schema validation
cat data.json | fluxmux-cli bridge \
    --source stdin \
    --sink stdout \
    --schema-path schema.json
```

### 3. Pipe Command
```bash
# Filter and transform
fluxmux-cli pipe file:data.json \
    filter 'temperature>30' \
    transform 'fahrenheit=temperature*1.8+32' \
    tee output.json

# Aggregate data
fluxmux-cli pipe file:sales.json \
    aggregate --group-by product --sum amount --avg price

# Chain multiple operations
cat data.json | fluxmux-cli pipe stdin \
    filter 'status=="active"' \
    transform 'total=price*quantity' \
    limit 100 \
    tee file:results.json stdout
```

### 4. Kafka Command
```bash
# Show first 10 messages
fluxmux-cli kafka --topic orders --broker localhost:9092 --head 10

# Monitor latest 5 messages (live)
fluxmux-cli kafka --topic logs --broker localhost:9092 --tail 5

# Use custom consumer group
fluxmux-cli kafka --topic events --broker localhost:9092 --group mygroup --head 20
```

## Requirements

### Installed
- ✅ Rust 1.91.0 (stable)
- ✅ Cargo build system
- ✅ CMake (for rdkafka)

### Optional Services (for specific features)
- Kafka (for Kafka source/sink operations)
- PostgreSQL (for PostgreSQL sink operations)

## Project Structure
```
FluxMux/
├── crates/
│   ├── fluxmux-cli/        # Main CLI application
│   ├── fluxmux-core/       # Core engine and middleware
│   ├── fluxmux-connectors/ # Source connectors
│   ├── fluxmux-sinks/      # Sink implementations
│   ├── fluxmux-codecs/     # Format codecs
│   ├── fluxmux-plugins/    # Plugin system
│   └── fluxmux-ui/         # UI components
├── examples/
│   ├── bridge/             # Bridge examples
│   └── convert/            # Convert examples
└── target/
    ├── debug/              # Debug builds
    └── release/            # Release builds
```

## Troubleshooting

### If you see "cargo: command not found"
```bash
# Load Cargo environment
source "$HOME/.cargo/env"

# Or restart your terminal
```

### Clean rebuild
```bash
cargo clean
cargo build --release
```

### Check for errors
```bash
cargo check --workspace
```

### Update dependencies
```bash
cargo update
```

## Performance Tips

1. **Always use release builds for production**:
   ```bash
   cargo build --release
   ```

2. **Release builds are significantly faster** than debug builds

3. **For large datasets**, consider using:
   - Batching (`--batch-size`)
   - Throttling (`--throttle-per-sec`)
   - Appropriate buffer sizes

## Next Steps

1. ✅ Application is built and working
2. ✅ All tests pass
3. 📖 Read the documentation:
   - [README.md](README.md) - Overview
   - [BRIDGE_IMPLEMENTATION.md](BRIDGE_IMPLEMENTATION.md) - Bridge details
   - [PIPE_COMMAND.md](PIPE_COMMAND.md) - Pipe command details
   - [KAFKA_COMMAND.md](KAFKA_COMMAND.md) - Kafka inspection
4. 🚀 Start using FluxMux for your data pipeline needs!

## Support

For issues or questions:
1. Check the documentation files in this repository
2. Run commands with `--help` flag for detailed usage
3. Review examples in the `examples/` directory

---

**Status**: ✅ All systems operational
**Version**: 0.1.0
**Last Updated**: November 1, 2025
