# 🎉 FluxMux Complete Guide - Start Here!

## 📚 Documentation Overview

Welcome to **FluxMux** - your complete data streaming and transformation platform! This guide will help you get started with real-time data processing.

---

## 🚀 Quick Start (3 Steps)

### Step 1: Install Kafka (Required)

**Easiest Method - Docker:**
```bash
# Download Docker Desktop: https://www.docker.com/products/docker-desktop

# Start Kafka (docker-compose.yml already in project)
docker-compose up -d

# Verify
docker-compose ps
```

**Alternative - Homebrew (macOS):**
```bash
brew install kafka
brew services start zookeeper
brew services start kafka
```

### Step 2: Run Automated Demo
```bash
chmod +x demo-realtime.sh
./demo-realtime.sh
```

This will:
- Test all features (Convert, Bridge, Pipe, Kafka)
- Create sample data files
- Stream real-time events
- Show you everything FluxMux can do!

### Step 3: Start Web GUI
```bash
# Terminal 1: Backend
cd fluxmux-backend
npm start

# Terminal 2: Frontend
cd fluxmux-frontend
npm start

# Browser opens automatically to http://localhost:3000
```

---

## 📖 Documentation Files

### 🔥 **START HERE**
1. **[REALTIME_WORKFLOW.md](./REALTIME_WORKFLOW.md)** ⭐ **MUST READ**
   - Complete workflow guide
   - Prerequisites setup (Kafka, PostgreSQL)
   - 6 real-time workflow examples
   - Web GUI usage instructions
   - Advanced scenarios
   - Troubleshooting guide
   - **This is your main reference!**

### 📋 Quick References
2. **[QUICK_REFERENCE.md](./QUICK_REFERENCE.md)** 
   - All commands in one place
   - Quick copy-paste examples
   - System status checks
   - Troubleshooting tips
   - **Keep this handy!**

3. **[README.md](./README.md)**
   - Project overview
   - Features list
   - Basic usage

### 🏗️ Technical Deep Dives
4. **[ARCHITECTURE.md](./ARCHITECTURE.md)**
   - System architecture diagrams
   - Data flow visualizations
   - Component responsibilities
   - Performance characteristics
   - **For understanding internals**

5. **[KAFKA_IMPLEMENTATION.md](./KAFKA_IMPLEMENTATION.md)**
   - Kafka integration details
   - Inspector implementation
   - Producer/Consumer patterns

6. **[BRIDGE_IMPLEMENTATION.md](./BRIDGE_IMPLEMENTATION.md)**
   - Bridge pipeline details
   - Middleware stack
   - Connector/Sink types

### 📝 Setup Guides
7. **[COMPLETE.md](./COMPLETE.md)**
   - Full implementation summary
   - All files created
   - Installation steps

8. **[GUI_SETUP.md](./GUI_SETUP.md)**
   - Frontend/Backend setup
   - Component details

9. **[QUICKSTART.md](./QUICKSTART.md)**
   - Basic getting started guide

---

## 🔗 Essential Links

### Required Software

| Software | Download Link | Purpose |
|----------|--------------|---------|
| **Docker Desktop** | https://www.docker.com/products/docker-desktop | Easiest way to run Kafka |
| **Apache Kafka** | https://kafka.apache.org/downloads | Message streaming platform |
| **PostgreSQL** | https://www.postgresql.org/download/ | Database sink (optional) |
| Homebrew (macOS) | https://brew.sh/ | Package manager |
| **ngrok** | https://ngrok.com/ | Expose local Kafka to internet |

### Cloud Kafka Services (Real-Time Testing) ☁️

| Service | Link | Free Tier | Best For |
|---------|------|-----------|----------|
| **Upstash Kafka** ⭐ | https://upstash.com/ | 10K msg/day | REST API, easiest setup |
| **Confluent Cloud** ⭐ | https://www.confluent.io/confluent-cloud/tryfree/ | 400 MB/month | Full Kafka compatibility |
| **CloudKarafka** | https://www.cloudkarafka.com/ | 5 MB storage | Learning Kafka |
| **Redpanda Cloud** | https://redpanda.com/try-redpanda | 100 MB/s | High performance |
| **Aiven Kafka** | https://aiven.io/kafka | 30-day trial | Professional testing |

**See [CLOUD_KAFKA_SERVICES.md](./CLOUD_KAFKA_SERVICES.md) for detailed setup!**

### Learning Resources

| Topic | Link | Description |
|-------|------|-------------|
| **Kafka Tutorial** | https://www.conduktor.io/kafka/kafka-tutorials/ | Learn Kafka basics |
| Kafka Official Docs | https://kafka.apache.org/documentation/ | Complete reference |
| Stream Processing | https://www.confluent.io/learn/stream-processing/ | Concepts |
| Event-Driven Architecture | https://martinfowler.com/articles/201701-event-driven.html | Design patterns |

### Data Format Specs

| Format | Link | Use Case |
|--------|------|----------|
| **JSON** | https://www.json.org/ | Human-readable, widely supported |
| **YAML** | https://yaml.org/ | Configuration files |
| **TOML** | https://toml.io/ | Config files, simpler than YAML |
| CSV | Standard | Tabular data, Excel |
| **Parquet** | https://parquet.apache.org/ | Big data, columnar storage |
| **Avro** | https://avro.apache.org/ | Binary, schema evolution |
| MessagePack | https://msgpack.org/ | Efficient binary JSON |
| CBOR | https://cbor.io/ | Compact binary |

---

## 🎬 Real-Time Workflows

### Workflow 1: File Format Conversion
**Goal:** Convert data between formats
```bash
# JSON → YAML → TOML → CSV
./target/release/fluxmux-cli convert input.json output.yaml --from json --to yaml
```
**Web GUI:** Go to **Convert** page → Upload → Select format → Convert

---

### Workflow 2: Real-Time Kafka Streaming
**Goal:** Monitor live Kafka topics
```bash
# In Web GUI:
# 1. Go to Kafka Inspector page
# 2. Enter broker: localhost:9092
# 3. Enter topic: user-events
# 4. Select mode: Tail
# 5. Count: 10
# 6. Click "Start Live Monitoring" 🔴
# Messages auto-refresh every 2 seconds!
```

---

### Workflow 3: Data Pipeline (File → Kafka)
**Goal:** Stream data from file to Kafka with reliability
```bash
./target/release/fluxmux-cli bridge \
  --source file:data.json \
  --sink kafka://localhost:9092/my-topic \
  --batch-size 10 \
  --retry-max-attempts 3 \
  --throttle-per-sec 50
```
**Web GUI:** Go to **Bridge** page → Configure source/sink → Enable middleware → Run

---

### Workflow 4: Data Transformations
**Goal:** Filter and transform streaming data
```bash
./target/release/fluxmux-cli pipe file:users.json \
  filter 'age>18' \
  transform 'adult=true' \
  tee adults.json
```
**Web GUI:** Go to **Pipe** page → Add operations → Execute

---

### Workflow 5: End-to-End Pipeline
**Goal:** Complete streaming pipeline
```bash
# Producer: Generate events
./stream-producer.sh &

# FluxMux: Process and route
./target/release/fluxmux-cli bridge \
  --source kafka://localhost:9092/input \
  --sink kafka://localhost:9092/output \
  --batch-size 10

# Consumer: Monitor output
kafka-console-consumer --bootstrap-server localhost:9092 --topic output
```

---

## 🎯 Feature Matrix

| Feature | CLI | Web GUI | Real-Time | Description |
|---------|-----|---------|-----------|-------------|
| **Convert** | ✓ | ✓ | ✗ | Format conversion (8 formats) |
| **Bridge** | ✓ | ✓ | ✓ | Data pipelines with middleware |
| **Pipe** | ✓ | ✓ | ✓ | Transformations & filters |
| **Kafka Inspector** | ✓ | ✓ | ✓ | Real-time topic monitoring 🔴 |
| **Batching** | ✓ | ✓ | ✓ | Group messages for efficiency |
| **Retry** | ✓ | ✓ | ✓ | Automatic retries on failure |
| **Throttle** | ✓ | ✓ | ✓ | Rate limiting |
| **PostgreSQL Sink** | ✓ | ✓ | ✓ | Write to database |

---

## 📊 Supported Formats

### Text Formats
- **JSON** - JavaScript Object Notation (human-readable)
- **YAML** - YAML Ain't Markup Language (config files)
- **TOML** - Tom's Obvious Minimal Language (simpler config)
- **CSV** - Comma-Separated Values (Excel, spreadsheets)

### Binary Formats
- **Parquet** - Columnar storage (big data, analytics)
- **Avro** - Binary with schema (Kafka, Hadoop)
- **MessagePack** - Efficient binary JSON
- **CBOR** - Concise Binary Object Representation

---

## 🔌 Connectors & Sinks

### Sources (Read From)
- ✅ **File** - Local/network files
- ✅ **Kafka** - Kafka topics
- ✅ **Pipe** - Standard input stream

### Sinks (Write To)
- ✅ **File** - Local/network files
- ✅ **Kafka** - Kafka topics
- ✅ **Pipe** - Standard output stream
- ✅ **PostgreSQL** - Database tables

---

## 🛠️ Common Commands

### System Setup
```bash
# Check Kafka is running
nc -z localhost 9092 && echo "✓ Kafka running" || echo "✗ Kafka not running"

# Start Kafka (Docker)
docker-compose up -d

# Build FluxMux CLI
cargo build --release

# Start Web GUI
./start-gui.sh
```

### Kafka Operations
```bash
# Create topic
kafka-topics --bootstrap-server localhost:9092 --create --topic my-topic --partitions 3 --replication-factor 1

# List topics
kafka-topics --bootstrap-server localhost:9092 --list

# Send messages
echo '{"test": "data"}' | kafka-console-producer --bootstrap-server localhost:9092 --topic my-topic

# Read messages
kafka-console-consumer --bootstrap-server localhost:9092 --topic my-topic --from-beginning
```

### FluxMux CLI
```bash
# Convert
./target/release/fluxmux-cli convert input.json output.yaml --from json --to yaml

# Bridge
./target/release/fluxmux-cli bridge --source file:data.json --sink kafka://localhost:9092/topic

# Pipe
./target/release/fluxmux-cli pipe file:input.json filter 'age>18' tee output.json

# Kafka Inspector
./target/release/fluxmux-cli kafka --topic my-topic --broker localhost:9092 --tail 10
```

---

## 🔍 Troubleshooting

### Problem: Kafka not starting
**Solution:**
```bash
# Check Docker containers
docker-compose ps
docker-compose logs kafka

# Restart
docker-compose down && docker-compose up -d
```

### Problem: Backend not connecting to CLI
**Solution:**
```bash
# Verify CLI binary exists
ls -lh target/release/fluxmux-cli

# Rebuild if needed
cargo build --release
```

### Problem: Frontend won't compile
**Solution:**
```bash
cd fluxmux-frontend
rm -rf node_modules package-lock.json
npm install
npm start
```

### Problem: Messages not appearing in Kafka
**Solution:**
```bash
# Verify topic exists
kafka-topics --bootstrap-server localhost:9092 --list

# Check messages
kafka-console-consumer --bootstrap-server localhost:9092 --topic my-topic --from-beginning --max-messages 10
```

---

## 📱 Web GUI Pages

### 1. Home (http://localhost:3000)
- Dashboard with feature cards
- Quick navigation

### 2. Convert (http://localhost:3000/convert)
- Upload files (drag & drop)
- Select output format
- Preview & download results

### 3. Bridge (http://localhost:3000/bridge)
- Configure source & sink
- Enable middleware (batch, retry, throttle)
- View execution logs

### 4. Pipe (http://localhost:3000/pipe)
- Build transformation pipelines
- Add filters, transforms, maps
- Execute and view output

### 5. Kafka Inspector (http://localhost:3000/kafka) 🔴
- Browse topics
- Inspect messages (head/tail/range)
- **Real-time monitoring** (auto-refresh)

---

## 🎓 Learning Path

### For Beginners
1. Read **QUICK_REFERENCE.md** - Get familiar with commands
2. Run **demo-realtime.sh** - See everything in action
3. Try **Web GUI** - Use each feature visually
4. Read **REALTIME_WORKFLOW.md** - Understand workflows

### For Developers
1. Read **ARCHITECTURE.md** - System design
2. Explore source code in `crates/`
3. Read **KAFKA_IMPLEMENTATION.md** - Kafka integration
4. Read **BRIDGE_IMPLEMENTATION.md** - Pipeline details

### For DevOps
1. Setup Kafka cluster (3+ brokers)
2. Configure production deployment
3. Setup monitoring (Prometheus, Grafana)
4. Implement CI/CD pipelines

---

## 📈 Performance Tips

### High Throughput
```bash
# Increase batch size
--batch-size 100

# Increase Kafka partitions
kafka-topics --bootstrap-server localhost:9092 --alter --topic my-topic --partitions 10

# Use multiple consumers
```

### Low Latency
```bash
# Reduce batch size
--batch-size 1

# Remove throttling
# (don't use --throttle-per-sec)

# Use direct connection
```

### Reliability
```bash
# Enable retries
--retry-max-attempts 3
--retry-delay-ms 1000

# Use acknowledgments
# (Kafka producer config)

# Monitor with Kafka Inspector
```

---

## 🎯 Use Cases

### 1. Log Aggregation
**Scenario:** Collect logs from multiple sources
```bash
# File → Kafka → Elasticsearch
./target/release/fluxmux-cli bridge \
  --source file:app.log \
  --sink kafka://localhost:9092/logs \
  --batch-size 100
```

### 2. Data Migration
**Scenario:** Migrate data between databases
```bash
# PostgreSQL → Kafka → MongoDB
# (PostgreSQL source coming soon)
```

### 3. Real-Time Analytics
**Scenario:** Process streaming data
```bash
# Kafka → Transform → Analytics DB
./target/release/fluxmux-cli pipe kafka://localhost:9092/events \
  filter 'amount>1000' \
  transform 'category="high-value"' \
  tee analytics.json
```

### 4. Format Standardization
**Scenario:** Convert legacy formats to modern ones
```bash
# CSV → JSON (for APIs)
./target/release/fluxmux-cli convert legacy.csv modern.json --from csv --to json
```

### 5. Event Streaming
**Scenario:** Real-time event processing
```bash
# Monitor user events in real-time
# Web GUI → Kafka Inspector → Tail mode 🔴
```

---

## 🎬 Demo Script Output

When you run `./demo-realtime.sh`, you'll see:

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   FluxMux Real-Time Demo - All Features
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✓ Setup complete

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
DEMO 1: Data Format Conversion
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Creating sample JSON data...
✓ Created users.json
Converting JSON → YAML...
✓ Converted to YAML
Converting YAML → TOML...
✓ Converted to TOML
Converting JSON → CSV...
✓ Converted to CSV

✅ DEMO 1 Complete: Format conversion successful

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
DEMO 2: Kafka Setup & Testing
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Checking Kafka connection...
✓ Kafka is running
Creating Kafka topics...
✓ Kafka topics created

... and more!
```

---

## 📞 Support & Resources

### Documentation
- All guides in project root (`.md` files)
- Start with **REALTIME_WORKFLOW.md**
- Use **QUICK_REFERENCE.md** for commands

### Community Resources
- Kafka Documentation: https://kafka.apache.org/documentation/
- Rust Documentation: https://doc.rust-lang.org/
- React Documentation: https://react.dev/

### Troubleshooting
- Check **REALTIME_WORKFLOW.md** → Troubleshooting section
- Check **QUICK_REFERENCE.md** → Troubleshooting section
- Verify all services running with status check

---

## 🎉 You're Ready!

### Next Steps:
1. ✅ Install Kafka: `docker-compose up -d`
2. ✅ Run demo: `./demo-realtime.sh`
3. ✅ Start GUI: `./start-gui.sh`
4. ✅ Open browser: http://localhost:3000
5. ✅ Try each feature!

### Remember:
- 📖 **REALTIME_WORKFLOW.md** is your main guide
- 📋 **QUICK_REFERENCE.md** for quick commands
- 🏗️ **ARCHITECTURE.md** for system design
- 🔗 Use links above to download required software

---

**Happy Streaming! 🚀✨**

**Version:** 1.0.0  
**Last Updated:** November 1, 2025  
**Project:** FluxMux - Real-Time Data Streaming & Transformation Platform
