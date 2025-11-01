# FluxMux 🚀

## Your all-in-one platform for real-time data streaming, transformation, and format conversion

**FluxMux** is a complete data processing platform with both CLI and Web GUI, supporting real-time streaming, format conversion, and production-grade data pipelines.

---

## 🌟 Quick Links

- **[🚀 START HERE](./START_HERE.md)** - Complete guide with all links and resources
- **[📖 Real-Time Workflows](./REALTIME_WORKFLOW.md)** - Main reference guide
- **[📋 Quick Reference](./QUICK_REFERENCE.md)** - All commands in one place
- **[🏗️ Architecture](./ARCHITECTURE.md)** - System design and data flow

---

## ✨ Features

### 🖥️ Web GUI (React + Express)
- **Convert** - Upload and convert files between 8 formats
- **Bridge** - Build data pipelines with middleware
- **Pipe** - Create transformation pipelines visually
- **Kafka Inspector** 🔴 - Real-time topic monitoring with live updates

### 💻 CLI (Rust)
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

## 📚 Documentation

### Getting Started
- **[START_HERE.md](./START_HERE.md)** ⭐ **Main entry point with all links**
- [REALTIME_WORKFLOW.md](./REALTIME_WORKFLOW.md) - Complete workflow guide
- [QUICK_REFERENCE.md](./QUICK_REFERENCE.md) - Quick command reference
- [QUICKSTART.md](./QUICKSTART.md) - Basic getting started

### Technical Documentation
- [ARCHITECTURE.md](./ARCHITECTURE.md) - System architecture and data flow
- [KAFKA_IMPLEMENTATION.md](./KAFKA_IMPLEMENTATION.md) - Kafka integration details
- [BRIDGE_IMPLEMENTATION.md](./BRIDGE_IMPLEMENTATION.md) - Bridge command reference
- [PIPE_COMMAND.md](./PIPE_COMMAND.md) - Pipe command reference

### Setup Guides
- [COMPLETE.md](./COMPLETE.md) - Full implementation summary
- [GUI_SETUP.md](./GUI_SETUP.md) - Frontend/Backend setup details

---

## 🚀 Quick Start

### Option 1: Automated (Recommended)
```bash
# 1. Install Kafka (Docker - easiest)
docker-compose up -d

# 2. Run real-time demo
./demo-realtime.sh

# 3. Start Web GUI
./start-gui.sh

# 4. Open browser
# http://localhost:3000
```

### Option 2: Manual Setup
```bash
# Terminal 1: Backend
cd fluxmux-backend
npm start

# Terminal 2: Frontend
cd fluxmux-frontend
npm start

# Browser opens to http://localhost:3000
```

---

## 🔗 Essential Links

### Required Software
- **Docker Desktop**: https://www.docker.com/products/docker-desktop (easiest Kafka setup)
- **Apache Kafka**: https://kafka.apache.org/downloads
- **PostgreSQL**: https://www.postgresql.org/download/ (optional)

### Learning Resources
- **Kafka Tutorial**: https://www.conduktor.io/kafka/kafka-tutorials/
- **Kafka Docs**: https://kafka.apache.org/documentation/
- **Stream Processing**: https://www.confluent.io/learn/stream-processing/

---

## 🎯 Supported Formats

**Text Formats:** JSON, YAML, TOML, CSV  
**Binary Formats:** Parquet, Avro, MessagePack, CBOR

---

## 🔌 Connectors & Sinks

**Sources:** File, Kafka, Pipe, stdin  
**Sinks:** File, Kafka, Pipe, PostgreSQL, stdout

---

## 🛠️ Requirements

- **Rust** (stable) - Already installed ✓
- **Node.js** - Already installed ✓
- **Kafka** - Install via Docker (see [START_HERE.md](./START_HERE.md))
- **CMake** - For rdkafka (Kafka client)
- **PostgreSQL** - Optional, for database sink

---

## 📈 Real-Time Features

- ✅ **Live Kafka Monitoring** - Auto-refresh every 2 seconds
- ✅ **Streaming Pipelines** - Process data in real-time
- ✅ **Middleware Stack** - Batching, Retry, Throttle
- ✅ **Format Conversion** - 8 formats supported
- ✅ **Data Transformations** - Filter, Transform, Map, Tee

---

## 🎬 Demo Script

Run the automated demo to see all features:
```bash
chmod +x demo-realtime.sh
./demo-realtime.sh
```

This demonstrates:
- Format conversions (JSON ↔ YAML ↔ TOML ↔ CSV)
- Kafka topic setup and streaming
- Bridge pipelines with middleware
- Pipe transformations
- Real-time monitoring

---

## 📱 Web GUI Pages

1. **Home** - Dashboard with feature overview
2. **Convert** - Format conversion (upload/download)
3. **Bridge** - Data pipeline builder
4. **Pipe** - Transformation pipeline builder
5. **Kafka Inspector** 🔴 - Real-time topic monitoring

---

## 🎓 Next Steps

1. **Read [START_HERE.md](./START_HERE.md)** - Complete guide
2. **Install Kafka** - `docker-compose up -d`
3. **Run demo** - `./demo-realtime.sh`
4. **Start GUI** - `./start-gui.sh`
5. **Explore features** - http://localhost:3000

---

## 📞 Support

- Check **[REALTIME_WORKFLOW.md](./REALTIME_WORKFLOW.md)** for detailed guides
- Check **[QUICK_REFERENCE.md](./QUICK_REFERENCE.md)** for quick commands
- Check **Troubleshooting** sections in docs

---

## 📄 License

MIT

---

**Happy Streaming! 🌊✨**