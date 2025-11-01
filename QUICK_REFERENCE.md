# 🎯 FluxMux Quick Reference Card

## 🚀 Start Everything

```bash
# Option 1: Automated (Recommended)
./start-gui.sh

# Option 2: Manual
# Terminal 1
cd fluxmux-backend && npm start

# Terminal 2  
cd fluxmux-frontend && npm start

# Browser opens to http://localhost:3000
```

---

## 📦 Install Kafka (Required for Bridge/Kafka features)

### Docker (Easiest) ⭐
```bash
# Download: https://www.docker.com/products/docker-desktop

# Create docker-compose.yml (already in project root)
docker-compose up -d              # Start
docker-compose ps                 # Check status
docker-compose logs -f kafka      # View logs
docker-compose down               # Stop
```

### Homebrew (macOS)
```bash
brew install kafka
brew services start zookeeper
brew services start kafka
```

### Manual
```bash
# Download: https://kafka.apache.org/downloads
# Extract and run:
bin/zookeeper-server-start.sh config/zookeeper.properties  # Terminal 1
bin/kafka-server-start.sh config/server.properties         # Terminal 2
```

---

## 🔬 Run Real-Time Demo

```bash
./demo-realtime.sh
```

**What it does:**
- ✅ Tests format conversions (JSON ↔ YAML ↔ TOML ↔ CSV)
- ✅ Creates Kafka topics
- ✅ Streams real-time events
- ✅ Runs bridge pipelines
- ✅ Executes transformations
- ✅ Generates sample data files in `./demo-data/`

---

## 🌐 Web GUI Features

### 1. **Convert** (http://localhost:3000/convert)
- Upload: JSON, YAML, TOML, CSV
- Convert to any format
- Download results

**Example:**
```
Upload: users.json → Convert to: YAML → Download
```

---

### 2. **Bridge** (http://localhost:3000/bridge)
- Source → Sink pipeline
- Middleware: Batching, Retry, Throttle

**Example:**
```
Source: file:data.json
Sink: kafka://localhost:9092/my-topic
Middleware:
  ☑ Batching: 10
  ☑ Retry: 3 attempts
  ☑ Throttle: 50/sec
```

---

### 3. **Pipe** (http://localhost:3000/pipe)
- Build transformation pipelines
- Filter, Transform, Map, Tee

**Example:**
```
Source: file:users.json
Filter: age > 18
Transform: adult = true
Tee: adults.json
```

---

### 4. **Kafka Inspector** (http://localhost:3000/kafka) 🔴 LIVE
- Browse topics
- Inspect messages (head/tail/range)
- **Real-time monitoring**

**Example:**
```
Broker: localhost:9092
Topic: user-events
Mode: Tail
Count: 10
[Start Live Monitoring] → Auto-refresh every 2s
```

---

## 💻 CLI Quick Commands

### Convert Files
```bash
./target/release/fluxmux-cli convert input.json output.yaml --from json --to yaml
./target/release/fluxmux-cli convert input.yaml output.toml --from yaml --to toml
./target/release/fluxmux-cli convert input.json output.csv --from json --to csv
```

### Bridge Pipeline
```bash
# File → Kafka
./target/release/fluxmux-cli bridge \
  --source file:data.json \
  --sink kafka://localhost:9092/my-topic \
  --batch-size 10 \
  --retry-max-attempts 3 \
  --throttle-per-sec 50

# Kafka → Kafka
./target/release/fluxmux-cli bridge \
  --source kafka://localhost:9092/input-topic \
  --sink kafka://localhost:9092/output-topic
```

### Pipe Transformations
```bash
./target/release/fluxmux-cli pipe file:input.json \
  filter 'age>18' \
  transform 'adult=true' \
  tee output.json
```

### Kafka Inspector
```bash
# Read first 10 messages
./target/release/fluxmux-cli kafka --topic my-topic --broker localhost:9092 --head 10

# Read last 10 messages
./target/release/fluxmux-cli kafka --topic my-topic --broker localhost:9092 --tail 10

# Read message range
./target/release/fluxmux-cli kafka --topic my-topic --broker localhost:9092 --start 0 --end 100
```

---

## 🐘 Kafka Commands

### Topics
```bash
# Create topic
kafka-topics --bootstrap-server localhost:9092 \
  --create --topic my-topic \
  --partitions 3 --replication-factor 1

# List topics
kafka-topics --bootstrap-server localhost:9092 --list

# Describe topic
kafka-topics --bootstrap-server localhost:9092 --describe --topic my-topic

# Delete topic
kafka-topics --bootstrap-server localhost:9092 --delete --topic my-topic
```

### Producer
```bash
# Send messages interactively
kafka-console-producer --bootstrap-server localhost:9092 --topic my-topic

# Send from file
cat data.json | kafka-console-producer --bootstrap-server localhost:9092 --topic my-topic

# Send single message
echo '{"test": "data"}' | kafka-console-producer --bootstrap-server localhost:9092 --topic my-topic
```

### Consumer
```bash
# Read from beginning
kafka-console-consumer --bootstrap-server localhost:9092 --topic my-topic --from-beginning

# Read latest messages
kafka-console-consumer --bootstrap-server localhost:9092 --topic my-topic

# Read N messages
kafka-console-consumer --bootstrap-server localhost:9092 --topic my-topic --max-messages 10
```

---

## 🎬 Real-Time Workflows

### Workflow 1: Streaming Data Pipeline
```bash
# Terminal 1: Start producer
cat > stream-data.sh << 'SCRIPT'
#!/bin/bash
counter=1
while true; do
  echo "{\"id\": $counter, \"timestamp\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"}" | \
    kafka-console-producer --bootstrap-server localhost:9092 --topic events 2>/dev/null
  echo "Sent event $counter"
  ((counter++))
  sleep 1
done
SCRIPT
chmod +x stream-data.sh
./stream-data.sh

# Terminal 2: Monitor in Web GUI
# Go to: http://localhost:3000/kafka
# Topic: events
# Mode: Tail
# [Start Live Monitoring]
```

### Workflow 2: File Conversion Stream
```bash
# Create input
echo '[{"name": "Alice", "age": 30}]' > input.json

# Convert
./target/release/fluxmux-cli convert input.json output.yaml --from json --to yaml

# Verify
cat output.yaml
```

### Workflow 3: Bridge with Middleware
```bash
# Create data
echo '{"sensor": "temp-1", "value": 23.5}' > sensors.json

# Run bridge
./target/release/fluxmux-cli bridge \
  --source file:sensors.json \
  --sink kafka://localhost:9092/sensors \
  --batch-size 5 \
  --retry-max-attempts 3 \
  --throttle-per-sec 10

# Verify
kafka-console-consumer --bootstrap-server localhost:9092 --topic sensors --from-beginning
```

---

## 🔧 Troubleshooting

### Kafka Not Running
```bash
# Check connection
nc -z localhost 9092

# Docker: Check status
docker-compose ps
docker-compose logs kafka

# Restart
docker-compose down && docker-compose up -d

# Homebrew: Restart
brew services restart zookeeper
brew services restart kafka
```

### Backend Issues
```bash
# Check backend is running
curl http://localhost:3001/api/convert

# Restart backend
cd fluxmux-backend
npm start
```

### Frontend Issues
```bash
# Clear cache and rebuild
cd fluxmux-frontend
rm -rf node_modules package-lock.json
npm install
npm start
```

### CLI Not Found
```bash
# Build CLI
cargo build --release

# Verify binary
ls -lh target/release/fluxmux-cli

# Test CLI
./target/release/fluxmux-cli --version
```

---

## 📊 System Status Check

```bash
# Check all services
echo "FluxMux CLI:"
./target/release/fluxmux-cli --version 2>&1 | head -1

echo -e "\nKafka:"
nc -z localhost 9092 && echo "✓ Running" || echo "✗ Not running"

echo -e "\nBackend:"
curl -s http://localhost:3001/api/convert > /dev/null && echo "✓ Running" || echo "✗ Not running"

echo -e "\nFrontend:"
curl -s http://localhost:3000 > /dev/null && echo "✓ Running" || echo "✗ Not running"
```

---

## 📚 Documentation Files

- **`README.md`** - Project overview
- **`REALTIME_WORKFLOW.md`** - Complete workflow guide (⭐ START HERE)
- **`COMPLETE.md`** - Implementation summary
- **`GUI_SETUP.md`** - GUI setup details
- **`QUICKSTART.md`** - Quick start guide
- **`KAFKA_IMPLEMENTATION.md`** - Kafka details
- **`BRIDGE_IMPLEMENTATION.md`** - Bridge details

---

## 🔗 Useful Links

### Required Software
- **Docker Desktop:** https://www.docker.com/products/docker-desktop
- **Kafka Downloads:** https://kafka.apache.org/downloads
- **PostgreSQL:** https://www.postgresql.org/download/
- **Rust:** https://www.rust-lang.org/tools/install (already installed)
- **Node.js:** https://nodejs.org/ (already installed)

### Learning Resources
- **Kafka Tutorial:** https://www.conduktor.io/kafka/kafka-tutorials/
- **Kafka Docs:** https://kafka.apache.org/documentation/
- **Stream Processing:** https://www.confluent.io/learn/stream-processing/
- **Event-Driven Architecture:** https://martinfowler.com/articles/201701-event-driven.html

### Data Formats
- **JSON:** https://www.json.org/
- **YAML:** https://yaml.org/
- **TOML:** https://toml.io/
- **Parquet:** https://parquet.apache.org/
- **Avro:** https://avro.apache.org/

---

## 🎯 Next Steps

1. **Install Kafka** (if not done):
   ```bash
   docker-compose up -d
   ```

2. **Run Demo**:
   ```bash
   ./demo-realtime.sh
   ```

3. **Start GUI**:
   ```bash
   ./start-gui.sh
   ```

4. **Open Browser**:
   ```
   http://localhost:3000
   ```

5. **Try Each Feature**:
   - Convert: Upload and convert files
   - Bridge: Create data pipelines
   - Pipe: Build transformations
   - Kafka: Monitor live streams 🔴

---

## 📞 Support

If you encounter issues:
1. Check **Troubleshooting** section above
2. Read **`REALTIME_WORKFLOW.md`** for detailed guide
3. Verify all services are running (Status Check)
4. Check terminal logs for errors

---

**Happy Streaming! 🚀✨**

---

## 🎓 Key Concepts

### Bridge
**Purpose:** Move data from source to sink with reliability  
**Features:** Batching, Retry, Throttle, Multiple connectors  
**Use Case:** Production data pipelines

### Pipe
**Purpose:** Transform and filter data  
**Features:** Filter, Transform, Map, Tee operations  
**Use Case:** Data processing and enrichment

### Kafka Inspector
**Purpose:** Debug and monitor Kafka topics  
**Features:** Head, Tail, Range inspection, Live monitoring  
**Use Case:** Real-time monitoring and debugging

### Middleware
- **Batching:** Group messages for efficiency
- **Retry:** Auto-retry on failures
- **Throttle:** Rate limiting to prevent overload

---

**Version:** 1.0.0  
**Last Updated:** November 1, 2025
