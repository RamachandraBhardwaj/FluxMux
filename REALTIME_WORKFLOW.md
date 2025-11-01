# 🚀 FluxMux Real-Time Data Workflow Guide

Complete guide to running FluxMux with real-time streaming data across all features.

---

## 📚 Table of Contents

1. [Quick Start](#quick-start)
2. [Prerequisites Setup](#prerequisites-setup)
3. [Real-Time Workflows](#real-time-workflows)
4. [Web GUI Usage](#web-gui-usage)
5. [Advanced Scenarios](#advanced-scenarios)
6. [Troubleshooting](#troubleshooting)

---

## ⚡ Quick Start

### Automated Demo (Recommended)

```bash
# Run the automated demo script
chmod +x demo-realtime.sh
./demo-realtime.sh
```

This will:
- ✅ Test format conversions
- ✅ Setup Kafka topics
- ✅ Stream real-time events
- ✅ Run bridge pipelines
- ✅ Execute transformations
- ✅ Generate sample data files

### Manual Start

```bash
# Terminal 1: Start backend
cd fluxmux-backend
npm start

# Terminal 2: Start frontend
cd fluxmux-frontend
npm start

# Browser opens automatically to http://localhost:3000
```

---

## 🔧 Prerequisites Setup

### 1. Install Kafka (Required for Bridge & Kafka Features)

#### Option A: Docker (Easiest) ⭐ **RECOMMENDED**

```bash
# Create docker-compose.yml in project root
cat > docker-compose.yml << 'EOF'
version: '3'
services:
  zookeeper:
    image: confluentinc/cp-zookeeper:latest
    environment:
      ZOOKEEPER_CLIENT_PORT: 2181
      ZOOKEEPER_TICK_TIME: 2000
    ports:
      - "2181:2181"

  kafka:
    image: confluentinc/cp-kafka:latest
    depends_on:
      - zookeeper
    ports:
      - "9092:9092"
    environment:
      KAFKA_BROKER_ID: 1
      KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://localhost:9092
      KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1
      KAFKA_LOG_RETENTION_HOURS: 1
      KAFKA_LOG_SEGMENT_BYTES: 1073741824
EOF

# Start Kafka
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f kafka

# Stop Kafka
docker-compose down
```

**Docker Desktop Download:** https://www.docker.com/products/docker-desktop

#### Option B: Homebrew (macOS)

```bash
# Install Kafka
brew install kafka

# Start ZooKeeper
brew services start zookeeper

# Start Kafka
brew services start kafka

# Verify
kafka-topics --bootstrap-server localhost:9092 --list

# Stop services
brew services stop kafka
brew services stop zookeeper
```

**Homebrew Download:** https://brew.sh/

#### Option C: Manual Install

1. **Download Kafka:**
   - Link: https://kafka.apache.org/downloads
   - Get latest version (e.g., kafka_2.13-3.6.0.tgz)

2. **Extract:**
   ```bash
   tar -xzf kafka_2.13-3.6.0.tgz
   cd kafka_2.13-3.6.0
   ```

3. **Start ZooKeeper:**
   ```bash
   bin/zookeeper-server-start.sh config/zookeeper.properties
   ```

4. **Start Kafka (new terminal):**
   ```bash
   bin/kafka-server-start.sh config/server.properties
   ```

### 2. Install PostgreSQL (Optional - For Database Sink)

#### Docker (Easiest)

```bash
docker run -d \
  --name fluxmux-postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=fluxmux_test \
  -p 5432:5432 \
  postgres:latest

# Connect to database
docker exec -it fluxmux-postgres psql -U postgres -d fluxmux_test
```

#### Homebrew (macOS)

```bash
# Install
brew install postgresql

# Start service
brew services start postgresql

# Create database
createdb fluxmux_test

# Connect
psql fluxmux_test
```

**PostgreSQL Downloads:**
- Official: https://www.postgresql.org/download/
- Docker Hub: https://hub.docker.com/_/postgres

### 3. Verify Installation

```bash
# Check Kafka
nc -z localhost 9092 && echo "✓ Kafka is running" || echo "✗ Kafka is not running"

# Check PostgreSQL (if installed)
nc -z localhost 5432 && echo "✓ PostgreSQL is running" || echo "✗ PostgreSQL is not running"

# Check FluxMux CLI
./target/release/fluxmux-cli --version
```

---

## 🌊 Real-Time Workflows

### Workflow 1: File Format Conversion Stream

**Use Case:** Convert incoming data files between formats in real-time.

```bash
# 1. Create sample JSON data
cat > input-stream.json << 'EOF'
[
  {"orderId": 1001, "product": "Laptop", "price": 999.99, "customer": "Alice"},
  {"orderId": 1002, "product": "Mouse", "price": 29.99, "customer": "Bob"},
  {"orderId": 1003, "product": "Keyboard", "price": 79.99, "customer": "Carol"}
]
EOF

# 2. Convert to different formats
./target/release/fluxmux-cli convert input-stream.json output.yaml --from json --to yaml
./target/release/fluxmux-cli convert input-stream.json output.toml --from json --to toml
./target/release/fluxmux-cli convert input-stream.json output.csv --from json --to csv

# 3. Verify outputs
cat output.yaml
cat output.toml
cat output.csv
```

**Web GUI Workflow:**
1. Navigate to **Convert** page
2. Select `input-stream.json`
3. Choose output format (YAML/TOML/CSV)
4. Click "Convert"
5. View/download result

---

### Workflow 2: Real-Time Kafka Streaming

**Use Case:** Monitor live events from Kafka topics.

```bash
# 1. Create Kafka topic
kafka-topics --bootstrap-server localhost:9092 \
  --create --topic live-events \
  --partitions 3 \
  --replication-factor 1

# 2. Start producer (simulating real-time events)
# Terminal 1: Producer
cat > stream-producer.sh << 'SCRIPT'
#!/bin/bash
counter=1
while true; do
  timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
  event="{\"eventId\": $counter, \"type\": \"user-action\", \"timestamp\": \"$timestamp\", \"data\": \"Event $counter\"}"
  echo "$event" | kafka-console-producer --bootstrap-server localhost:9092 --topic live-events 2>/dev/null
  echo "Sent event $counter"
  ((counter++))
  sleep 2
done
SCRIPT

chmod +x stream-producer.sh
./stream-producer.sh

# 3. Monitor with FluxMux (Terminal 2)
./target/release/fluxmux-cli kafka \
  --topic live-events \
  --broker localhost:9092 \
  --tail 10
```

**Web GUI Workflow:**
1. Navigate to **Kafka Inspector**
2. Enter broker: `localhost:9092`
3. Enter topic: `live-events`
4. Select mode: **Tail**
5. Set count: `10`
6. Click **Start Live Monitoring**
7. Watch messages appear in real-time! 🔴 **LIVE**

---

### Workflow 3: Bridge - File to Kafka Pipeline

**Use Case:** Stream data from files to Kafka with reliability features.

```bash
# 1. Create source data (simulating sensor readings)
cat > sensor-data.json << 'EOF'
{"sensorId": "temp-001", "value": 23.5, "unit": "celsius", "location": "warehouse-a"}
{"sensorId": "temp-002", "value": 25.1, "unit": "celsius", "location": "warehouse-b"}
{"sensorId": "humid-001", "value": 65.2, "unit": "percent", "location": "warehouse-a"}
{"sensorId": "pressure-001", "value": 1013.25, "unit": "hPa", "location": "factory-1"}
{"sensorId": "temp-003", "value": 24.8, "unit": "celsius", "location": "office"}
EOF

# 2. Create Kafka topic
kafka-topics --bootstrap-server localhost:9092 \
  --create --topic sensor-readings \
  --partitions 2 \
  --replication-factor 1

# 3. Run bridge with middleware
./target/release/fluxmux-cli bridge \
  --source file:sensor-data.json \
  --sink kafka://localhost:9092/sensor-readings \
  --batch-size 2 \
  --retry-max-attempts 3 \
  --retry-delay-ms 1000 \
  --throttle-per-sec 10

# 4. Verify data in Kafka
kafka-console-consumer \
  --bootstrap-server localhost:9092 \
  --topic sensor-readings \
  --from-beginning
```

**Web GUI Workflow:**
1. Navigate to **Bridge** page
2. **Source Configuration:**
   - Type: `file`
   - Path: `sensor-data.json`
3. **Sink Configuration:**
   - Type: `kafka`
   - Broker: `localhost:9092`
   - Topic: `sensor-readings`
4. **Middleware (optional):**
   - ✓ Enable Batching: Size = `2`
   - ✓ Enable Retry: Max attempts = `3`, Delay = `1000ms`
   - ✓ Enable Throttle: Rate = `10` per second
5. Click **Run Bridge**
6. View execution log and success message

---

### Workflow 4: Kafka to Kafka with Transformation

**Use Case:** Process messages from one topic and send to another.

```bash
# 1. Create topics
kafka-topics --bootstrap-server localhost:9092 --create --topic raw-data --partitions 1 --replication-factor 1
kafka-topics --bootstrap-server localhost:9092 --create --topic processed-data --partitions 1 --replication-factor 1

# 2. Send raw data
cat > raw-messages.json << 'EOF'
{"temperature": 25, "sensor": "A1"}
{"temperature": 30, "sensor": "B2"}
{"temperature": 22, "sensor": "C3"}
EOF

cat raw-messages.json | kafka-console-producer --bootstrap-server localhost:9092 --topic raw-data

# 3. Run bridge with transformation
./target/release/fluxmux-cli bridge \
  --source kafka://localhost:9092/raw-data \
  --sink kafka://localhost:9092/processed-data \
  --batch-size 1 \
  --throttle-per-sec 5

# 4. Consume processed data
kafka-console-consumer \
  --bootstrap-server localhost:9092 \
  --topic processed-data \
  --from-beginning
```

---

### Workflow 5: Pipe Transformations

**Use Case:** Filter and transform streaming data.

```bash
# 1. Create input data
cat > transactions.json << 'EOF'
{"txId": 1, "amount": 150, "status": "pending"}
{"txId": 2, "amount": 50, "status": "completed"}
{"txId": 3, "amount": 300, "status": "pending"}
{"txId": 4, "amount": 75, "status": "failed"}
{"txId": 5, "amount": 500, "status": "completed"}
EOF

# 2. Run pipe with filter and transform
./target/release/fluxmux-cli pipe file:transactions.json \
  filter 'amount>100' \
  transform 'fee=amount*0.02' \
  tee large-transactions.json

# 3. View filtered results
cat large-transactions.json
```

**Web GUI Workflow:**
1. Navigate to **Pipe** page
2. **Source:** `file:transactions.json`
3. **Add Operations:**
   - **Filter:** `amount > 100`
   - **Transform:** `fee = amount * 0.02`
   - **Tee:** `large-transactions.json`
4. Click **Execute Pipeline**
5. View output in console

---

### Workflow 6: End-to-End Streaming Pipeline

**Use Case:** Complete data pipeline from source → processing → multiple sinks.

```bash
# 1. Setup topics
kafka-topics --bootstrap-server localhost:9092 --create --topic orders --partitions 2 --replication-factor 1
kafka-topics --bootstrap-server localhost:9092 --create --topic high-value-orders --partitions 1 --replication-factor 1
kafka-topics --bootstrap-server localhost:9092 --create --topic analytics --partitions 1 --replication-factor 1

# 2. Start order producer (Terminal 1)
cat > order-stream.sh << 'SCRIPT'
#!/bin/bash
for i in {1..50}; do
  amount=$((RANDOM % 1000 + 50))
  status=("pending" "completed" "processing")
  status_val=${status[$RANDOM % ${#status[@]}]}
  order="{\"orderId\": $i, \"amount\": $amount, \"status\": \"$status_val\", \"timestamp\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"}"
  echo "$order" | kafka-console-producer --bootstrap-server localhost:9092 --topic orders 2>/dev/null
  echo "Order $i: \$$amount ($status_val)"
  sleep 1
done
SCRIPT
chmod +x order-stream.sh
./order-stream.sh &

# 3. Run processing pipeline (Terminal 2)
./target/release/fluxmux-cli bridge \
  --source kafka://localhost:9092/orders \
  --sink kafka://localhost:9092/analytics \
  --batch-size 5 \
  --throttle-per-sec 20

# 4. Monitor results (Terminal 3)
kafka-console-consumer --bootstrap-server localhost:9092 --topic analytics --from-beginning
```

**Architecture:**
```
Orders Source
      ↓
   [Kafka: orders]
      ↓
   Bridge Pipeline
   • Batching (5)
   • Throttle (20/sec)
      ↓
   [Kafka: analytics]
      ↓
   Analytics Dashboard
```

---

## 🖥️ Web GUI Usage

### Convert Page

**Features:**
- Upload files (JSON, YAML, TOML, CSV)
- Select output format
- Preview conversion
- Download result

**Example:**
1. Click **"Choose File"**
2. Select `users.json`
3. Choose **"Output Format: YAML"**
4. Click **"Convert"**
5. View converted data
6. Click **"Download"** to save

---

### Bridge Page

**Features:**
- Configure source (file, kafka, pipe)
- Configure sink (file, kafka, pipe, postgres)
- Enable middleware:
  - **Batching:** Group messages
  - **Retry:** Automatic retries on failure
  - **Throttle:** Rate limiting

**Example - File to Kafka:**
```
Source Type: file
Source Path: /path/to/data.json

Sink Type: kafka
Kafka Broker: localhost:9092
Kafka Topic: my-topic

Middleware:
☑ Batching: Size = 10
☑ Retry: Max = 3, Delay = 1000ms
☑ Throttle: 50 per second
```

Click **"Run Bridge"** → View logs in real-time

---

### Pipe Page

**Features:**
- Build transformation pipelines
- Visual pipeline builder
- Add/remove operations dynamically

**Example:**
```
Source: file:input.json

Operations:
1. Filter: age > 18
2. Transform: adult = true
3. Map: name = name.toUpperCase()
4. Tee: adults.json

Output: stdout
```

---

### Kafka Inspector Page

**Features:**
- List all topics
- Inspect messages (head/tail/range)
- Real-time monitoring
- Message search

**Live Monitoring Example:**
1. Enter **Broker:** `localhost:9092`
2. Enter **Topic:** `user-events`
3. Select **Mode:** Tail
4. Set **Count:** 10
5. Click **"Start Live Monitoring"**
6. Messages auto-refresh every 2 seconds 🔴

---

## 🎯 Advanced Scenarios

### Scenario 1: High-Volume Data Ingestion

**Goal:** Process 10,000 messages/sec from file to Kafka.

```bash
# Generate test data
cat > generate-data.sh << 'SCRIPT'
#!/bin/bash
for i in {1..10000}; do
  echo "{\"id\": $i, \"value\": $((RANDOM)), \"timestamp\": \"$(date -u +%s)\"}"
done > large-dataset.json
SCRIPT
chmod +x generate-data.sh
./generate-data.sh

# Run high-throughput bridge
./target/release/fluxmux-cli bridge \
  --source file:large-dataset.json \
  --sink kafka://localhost:9092/high-volume \
  --batch-size 100 \
  --throttle-per-sec 1000
```

**Performance Tips:**
- Increase batch size (100-1000)
- Increase Kafka partitions (10+)
- Use compression (Kafka producer config)
- Monitor with Kafka Inspector

---

### Scenario 2: Multi-Topic Fan-Out

**Goal:** Read from one topic, write to multiple topics based on conditions.

```bash
# Create topics
kafka-topics --bootstrap-server localhost:9092 --create --topic events --partitions 3 --replication-factor 1
kafka-topics --bootstrap-server localhost:9092 --create --topic errors --partitions 1 --replication-factor 1
kafka-topics --bootstrap-server localhost:9092 --create --topic success --partitions 2 --replication-factor 1

# Terminal 1: Route errors
./target/release/fluxmux-cli bridge \
  --source kafka://localhost:9092/events \
  --sink kafka://localhost:9092/errors

# Terminal 2: Route success
./target/release/fluxmux-cli bridge \
  --source kafka://localhost:9092/events \
  --sink kafka://localhost:9092/success
```

---

### Scenario 3: Data Enrichment Pipeline

**Goal:** Enrich messages with additional data.

```bash
# 1. Create lookup data
cat > users-lookup.json << 'EOF'
{"userId": 1, "name": "Alice", "tier": "premium"}
{"userId": 2, "name": "Bob", "tier": "basic"}
{"userId": 3, "name": "Carol", "tier": "premium"}
EOF

# 2. Create events
cat > user-events.json << 'EOF'
{"userId": 1, "action": "login"}
{"userId": 2, "action": "purchase"}
{"userId": 3, "action": "view"}
EOF

# 3. Run enrichment pipeline
./target/release/fluxmux-cli pipe file:user-events.json \
  transform 'enriched=true' \
  tee enriched-events.json

# In production, you'd join with lookup data
# FluxMux supports this through custom transforms
```

---

## 🔍 Troubleshooting

### Kafka Not Starting

**Error:** `Connection refused: localhost:9092`

**Solutions:**
```bash
# Check if running
nc -z localhost 9092

# Docker: Check container status
docker-compose ps
docker-compose logs kafka

# Restart Docker containers
docker-compose down
docker-compose up -d

# Homebrew: Restart services
brew services restart zookeeper
brew services restart kafka

# Check ZooKeeper first
nc -z localhost 2181
```

---

### Topic Already Exists

**Error:** `Topic 'xyz' already exists`

**Solution:**
```bash
# Delete topic
kafka-topics --bootstrap-server localhost:9092 --delete --topic xyz

# List all topics
kafka-topics --bootstrap-server localhost:9092 --list
```

---

### Messages Not Appearing

**Issue:** Bridge runs but no messages in Kafka.

**Debug Steps:**
```bash
# 1. Verify topic exists
kafka-topics --bootstrap-server localhost:9092 --list

# 2. Check messages count
kafka-run-class kafka.tools.GetOffsetShell \
  --broker-list localhost:9092 \
  --topic my-topic

# 3. Consume from beginning
kafka-console-consumer \
  --bootstrap-server localhost:9092 \
  --topic my-topic \
  --from-beginning \
  --max-messages 10

# 4. Check FluxMux logs
./target/release/fluxmux-cli bridge \
  --source file:test.json \
  --sink kafka://localhost:9092/my-topic \
  --verbose  # Add verbose flag
```

---

### Backend Not Connecting to CLI

**Error:** `Command not found: fluxmux-cli`

**Solution:**
```bash
# 1. Build CLI
cargo build --release

# 2. Verify binary exists
ls -lh target/release/fluxmux-cli

# 3. Update backend server.js path
# Edit: fluxmux-backend/server.js
# Change: const CLI_PATH = '../target/release/fluxmux-cli';

# 4. Restart backend
cd fluxmux-backend
npm start
```

---

### High Memory Usage

**Issue:** System slows down with large data.

**Solutions:**
```bash
# 1. Use batching to reduce memory
--batch-size 50

# 2. Add throttling
--throttle-per-sec 100

# 3. Process in chunks
split -l 1000 large-file.json chunk_

# 4. Use streaming mode (if available)
--streaming true
```

---

## 📊 Monitoring Dashboard

### Kafka Metrics

```bash
# Topic details
kafka-topics --bootstrap-server localhost:9092 \
  --describe --topic my-topic

# Consumer groups
kafka-consumer-groups --bootstrap-server localhost:9092 --list

# Consumer lag
kafka-consumer-groups --bootstrap-server localhost:9092 \
  --describe --group my-group
```

### FluxMux Metrics

Monitor in Web GUI:
- **Bridge:** Execution logs, message count
- **Kafka Inspector:** Message rate, lag
- **Pipe:** Transformation stats

---

## 🎓 Learning Resources

### Kafka
- Official Documentation: https://kafka.apache.org/documentation/
- Kafka Tutorial: https://www.conduktor.io/kafka/kafka-tutorials/
- Kafka Streams: https://kafka.apache.org/documentation/streams/

### Data Formats
- JSON: https://www.json.org/
- YAML: https://yaml.org/
- TOML: https://toml.io/
- Apache Parquet: https://parquet.apache.org/
- Apache Avro: https://avro.apache.org/

### Message Processing
- Stream Processing Concepts: https://www.confluent.io/learn/stream-processing/
- Event-Driven Architecture: https://martinfowler.com/articles/201701-event-driven.html

---

## 🎬 Quick Reference Commands

```bash
# Start Kafka (Docker)
docker-compose up -d

# Start FluxMux GUI
./start-gui.sh

# Run demo
./demo-realtime.sh

# Create topic
kafka-topics --bootstrap-server localhost:9092 --create --topic NAME --partitions 3 --replication-factor 1

# List topics
kafka-topics --bootstrap-server localhost:9092 --list

# Send messages
echo '{"test": "data"}' | kafka-console-producer --bootstrap-server localhost:9092 --topic NAME

# Read messages
kafka-console-consumer --bootstrap-server localhost:9092 --topic NAME --from-beginning

# Convert file
./target/release/fluxmux-cli convert input.json output.yaml --from json --to yaml

# Run bridge
./target/release/fluxmux-cli bridge --source file:in.json --sink kafka://localhost:9092/topic

# Inspect Kafka
./target/release/fluxmux-cli kafka --topic NAME --broker localhost:9092 --head 10
```

---

## 🚀 Production Deployment

### Best Practices

1. **Use Docker Compose** for services
2. **Configure Kafka** for durability:
   ```properties
   # server.properties
   log.retention.hours=168
   log.segment.bytes=1073741824
   num.partitions=3
   default.replication.factor=3
   ```
3. **Add monitoring** (Prometheus, Grafana)
4. **Setup alerts** for failures
5. **Use environment variables** for configuration
6. **Enable SSL/TLS** for security
7. **Implement authentication** (SASL, mTLS)

---

## 🎉 Summary

You now have:
- ✅ Kafka running locally
- ✅ FluxMux CLI built
- ✅ Web GUI running
- ✅ Real-time demo script
- ✅ Sample workflows
- ✅ Troubleshooting guide

**Next Steps:**
1. Run `./demo-realtime.sh` to see everything in action
2. Open Web GUI and explore each feature
3. Create your own data pipelines
4. Monitor real-time streams
5. Build production workflows

**Happy Streaming! 🌊✨**
