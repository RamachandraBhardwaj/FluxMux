# Fluxmux

## Your all-in-one CLI for message queues, streams, databases with file conversions and Unix-style data pipelines

FluxMux provides four powerful commands for data processing:

### 1. convert
Convert between data formats (JSON, YAML, TOML, CSV).

```powershell
fluxmux convert input.json output.yaml --from json --to yaml
```

### 2. bridge
Production-ready data pipelines with middleware (batching, retry, throttling, schema validation).

```powershell
fluxmux bridge `
	--source file:input.json `
	--sink kafka://localhost:9092/topic `
	--batch-size 10 `
	--schema-path schema.json `
	--retry-max-attempts 3
```

**Supported endpoints**: File, Kafka, PostgreSQL, stdin, stdout

**Middleware**: Deduplication, Throttling, Batching, Retry, Schema Validation

See [BRIDGE_IMPLEMENTATION.md](BRIDGE_IMPLEMENTATION.md) for complete documentation.

### 3. pipe
Unix-style inline pipelines with transformation actions.

```powershell
# Filter, transform, and output
Get-Content data.json | fluxmux pipe stdin `
	filter 'temp>30' `
	transform 'fahrenheit=temp*1.8+32' `
	tee file:hot.json stdout

# Aggregate by group
fluxmux pipe file:sales.json `
	-- aggregate --group-by product --sum amount --avg price
```

**Actions**: filter, transform, aggregate, normalize, validate, limit, sample, tee

See [PIPE_COMMAND.md](PIPE_COMMAND.md) for complete documentation.

### 4. kafka
Fast, real-time Kafka topic inspection with minimal latency.

```powershell
# Show first 10 messages (with EOF placeholders)
fluxmux kafka --topic orders --head 10

# Monitor latest 5 messages live
fluxmux kafka --topic logs --tail 5
```

**Features**: EOF placeholders in head mode, live updates in tail mode (<100ms latency), clean terminal UI

See [KAFKA_COMMAND.md](KAFKA_COMMAND.md) for complete documentation.

## Quick Start

```powershell
# Build
cargo build --release

# Convert formats
cargo run -p fluxmux-cli -- convert input.json output.yaml --from json --to yaml

# Bridge with all middleware
cargo run -p fluxmux-cli -- bridge `
	--source file:input.json `
	--sink kafka://localhost:9092/topic `
	--batch-size 10 `
	--deduplicate `
	--throttle-per-sec 100 `
	--retry-max-attempts 3

# Pipe transformations
Get-Content data.json | cargo run -p fluxmux-cli -- pipe stdin `
	filter 'value>100' `
	transform 'double=value*2' `
	limit 50
```

## Features

- **Multiple Data Sources**: File, Kafka, PostgreSQL, stdin
- **Multiple Sinks**: File, Kafka, PostgreSQL, stdout
- **Format Support**: JSON, YAML, TOML, CSV, NDJSON
- **Production Middleware**: Batching, retry, throttling, deduplication, schema validation
- **Inline Transformations**: Filter, transform, aggregate, validate
- **Multi-Output**: Tee to multiple destinations simultaneously
- **Expression Language**: Math and comparison expressions for data transformation
- **Windows PowerShell**: Full support for pipeline integration

## Documentation

- [BRIDGE_IMPLEMENTATION.md](BRIDGE_IMPLEMENTATION.md) - Complete bridge command reference
- [PIPE_COMMAND.md](PIPE_COMMAND.md) - Complete pipe command reference with examples

## Requirements

- Rust (stable)
- CMake (for Kafka support via rdkafka)
- Optional: Kafka, PostgreSQL for respective endpoints

## License

MIT