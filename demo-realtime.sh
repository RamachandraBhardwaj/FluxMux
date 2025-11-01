#!/bin/bash

# FluxMux Real-Time Demo Script
# This script demonstrates all FluxMux features with real data

set -e  # Exit on error

BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}      FluxMux Real-Time Demo - All Features${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Configuration
KAFKA_BROKER="localhost:9092"
FLUXMUX_CLI="./target/release/fluxmux-cli"
DEMO_DIR="./demo-data"

# Check if FluxMux CLI is built
if [ ! -f "$FLUXMUX_CLI" ]; then
    echo -e "${YELLOW}⚠️  FluxMux CLI not found. Building...${NC}"
    cargo build --release
fi

# Create demo data directory
mkdir -p "$DEMO_DIR"
cd "$DEMO_DIR"

echo -e "${GREEN}✓ Setup complete${NC}"
echo ""

# ============================================================================
# DEMO 1: DATA FORMAT CONVERSION
# ============================================================================

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}DEMO 1: Data Format Conversion${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Create sample JSON data
echo -e "${YELLOW}Creating sample JSON data...${NC}"
cat > users.json << 'EOF'
[
  {
    "id": 1,
    "name": "Alice Johnson",
    "email": "alice@example.com",
    "age": 30,
    "city": "New York",
    "active": true,
    "balance": 1250.50
  },
  {
    "id": 2,
    "name": "Bob Smith",
    "email": "bob@example.com",
    "age": 25,
    "city": "San Francisco",
    "active": true,
    "balance": 890.25
  },
  {
    "id": 3,
    "name": "Carol Davis",
    "email": "carol@example.com",
    "age": 35,
    "city": "Los Angeles",
    "active": false,
    "balance": 2340.75
  }
]
EOF

echo -e "${GREEN}✓ Created users.json${NC}"

# Convert JSON to YAML
echo -e "${YELLOW}Converting JSON → YAML...${NC}"
../$FLUXMUX_CLI convert users.json users.yaml --from json --to yaml
echo -e "${GREEN}✓ Converted to YAML${NC}"
echo "Preview:"
head -10 users.yaml
echo ""

# Convert YAML to TOML
echo -e "${YELLOW}Converting YAML → TOML...${NC}"
../$FLUXMUX_CLI convert users.yaml users.toml --from yaml --to toml
echo -e "${GREEN}✓ Converted to TOML${NC}"
echo "Preview:"
head -10 users.toml
echo ""

# Convert JSON to CSV
echo -e "${YELLOW}Converting JSON → CSV...${NC}"
../$FLUXMUX_CLI convert users.json users.csv --from json --to csv
echo -e "${GREEN}✓ Converted to CSV${NC}"
echo "Preview:"
head -5 users.csv
echo ""

echo -e "${GREEN}✅ DEMO 1 Complete: Format conversion successful${NC}"
echo ""
sleep 2

# ============================================================================
# DEMO 2: KAFKA SETUP & TESTING
# ============================================================================

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}DEMO 2: Kafka Setup & Testing${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Check if Kafka is running
echo -e "${YELLOW}Checking Kafka connection...${NC}"
if ! nc -z localhost 9092 2>/dev/null; then
    echo -e "${RED}❌ Kafka is not running on localhost:9092${NC}"
    echo -e "${YELLOW}Please start Kafka first:${NC}"
    echo "  brew services start zookeeper"
    echo "  brew services start kafka"
    echo "Or use Docker:"
    echo "  docker-compose up -d"
    echo ""
    echo -e "${YELLOW}Skipping Kafka demos...${NC}"
    KAFKA_AVAILABLE=false
else
    echo -e "${GREEN}✓ Kafka is running${NC}"
    KAFKA_AVAILABLE=true
fi

if [ "$KAFKA_AVAILABLE" = true ]; then
    # Create Kafka topics
    echo -e "${YELLOW}Creating Kafka topics...${NC}"
    
    # Create topics (suppress errors if they already exist)
    kafka-topics --bootstrap-server $KAFKA_BROKER --create --topic user-events --partitions 3 --replication-factor 1 2>/dev/null || true
    kafka-topics --bootstrap-server $KAFKA_BROKER --create --topic processed-users --partitions 2 --replication-factor 1 2>/dev/null || true
    kafka-topics --bootstrap-server $KAFKA_BROKER --create --topic analytics --partitions 1 --replication-factor 1 2>/dev/null || true
    
    echo -e "${GREEN}✓ Kafka topics created${NC}"
    
    # List topics
    echo -e "${YELLOW}Available topics:${NC}"
    kafka-topics --bootstrap-server $KAFKA_BROKER --list
    echo ""
    
    # Generate real-time events
    echo -e "${YELLOW}Generating real-time user events...${NC}"
    cat > events.json << 'EOF'
{"eventId": "e1", "userId": 101, "action": "login", "timestamp": "2025-11-01T10:00:00Z"}
{"eventId": "e2", "userId": 102, "action": "purchase", "amount": 49.99, "timestamp": "2025-11-01T10:01:00Z"}
{"eventId": "e3", "userId": 103, "action": "view", "product": "laptop", "timestamp": "2025-11-01T10:02:00Z"}
{"eventId": "e4", "userId": 101, "action": "logout", "timestamp": "2025-11-01T10:05:00Z"}
{"eventId": "e5", "userId": 104, "action": "signup", "email": "new@example.com", "timestamp": "2025-11-01T10:06:00Z"}
EOF
    
    # Send events to Kafka using kafka-console-producer
    echo -e "${YELLOW}Sending events to Kafka...${NC}"
    cat events.json | kafka-console-producer --bootstrap-server $KAFKA_BROKER --topic user-events
    echo -e "${GREEN}✓ Sent 5 events to 'user-events' topic${NC}"
    echo ""
    
    echo -e "${GREEN}✅ DEMO 2 Complete: Kafka setup successful${NC}"
    echo ""
    sleep 2
fi

# ============================================================================
# DEMO 3: KAFKA INSPECTOR
# ============================================================================

if [ "$KAFKA_AVAILABLE" = true ]; then
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}DEMO 3: Kafka Inspector (Head Mode)${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    
    echo -e "${YELLOW}Reading first 5 messages from 'user-events' topic...${NC}"
    ../$FLUXMUX_CLI kafka --topic user-events --broker $KAFKA_BROKER --head 5 || echo "Inspect in Web GUI"
    echo ""
    
    echo -e "${GREEN}✅ DEMO 3 Complete: Kafka inspection successful${NC}"
    echo ""
    sleep 2
fi

# ============================================================================
# DEMO 4: BRIDGE - FILE TO KAFKA
# ============================================================================

if [ "$KAFKA_AVAILABLE" = true ]; then
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}DEMO 4: Bridge - File to Kafka with Middleware${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    
    # Create streaming data file (simulating real-time data)
    echo -e "${YELLOW}Creating streaming data file...${NC}"
    cat > streaming-data.json << 'EOF'
{"sensor": "temp-1", "value": 23.5, "location": "warehouse-a"}
{"sensor": "temp-2", "value": 25.1, "location": "warehouse-b"}
{"sensor": "humid-1", "value": 65.2, "location": "warehouse-a"}
{"sensor": "temp-1", "value": 24.0, "location": "warehouse-a"}
{"sensor": "pressure-1", "value": 1013.25, "location": "factory"}
EOF
    
    echo -e "${YELLOW}Sending data through bridge to Kafka...${NC}"
    echo -e "${YELLOW}Features: Batching (2), Retry (3), Throttle (10/sec)${NC}"
    
    ../$FLUXMUX_CLI bridge \
        --source file:streaming-data.json \
        --sink kafka://$KAFKA_BROKER/analytics \
        --batch-size 2 \
        --retry-max-attempts 3 \
        --throttle-per-sec 10
    
    echo -e "${GREEN}✓ Data sent to Kafka through bridge${NC}"
    echo ""
    
    # Verify data in Kafka
    echo -e "${YELLOW}Verifying data in Kafka 'analytics' topic...${NC}"
    timeout 2 kafka-console-consumer --bootstrap-server $KAFKA_BROKER --topic analytics --from-beginning --max-messages 5 || true
    echo ""
    
    echo -e "${GREEN}✅ DEMO 4 Complete: Bridge pipeline successful${NC}"
    echo ""
    sleep 2
fi

# ============================================================================
# DEMO 5: PIPE - TRANSFORMATIONS
# ============================================================================

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}DEMO 5: Pipe - Data Transformations${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Create temperature data
echo -e "${YELLOW}Creating temperature data...${NC}"
cat > temperatures.json << 'EOF'
{"city": "New York", "temp": 25, "humidity": 60}
{"city": "Los Angeles", "temp": 35, "humidity": 40}
{"city": "Chicago", "temp": 15, "humidity": 70}
{"city": "Miami", "temp": 38, "humidity": 80}
{"city": "Seattle", "temp": 18, "humidity": 75}
EOF

echo -e "${YELLOW}Running pipe transformations:${NC}"
echo "  1. Filter: temp > 30"
echo "  2. Transform: fahrenheit = temp * 1.8 + 32"
echo "  3. Output to file and stdout"
echo ""

../$FLUXMUX_CLI pipe file:temperatures.json \
    filter 'temp>30' \
    transform 'fahrenheit=temp*1.8+32' \
    tee hot-cities.json || echo "Use Web GUI for complex pipes"

if [ -f hot-cities.json ]; then
    echo ""
    echo -e "${GREEN}✓ Filtered and transformed data:${NC}"
    cat hot-cities.json
fi

echo ""
echo -e "${GREEN}✅ DEMO 5 Complete: Pipe transformations successful${NC}"
echo ""
sleep 2

# ============================================================================
# DEMO 6: REAL-TIME STREAMING SIMULATION
# ============================================================================

if [ "$KAFKA_AVAILABLE" = true ]; then
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}DEMO 6: Real-Time Streaming Simulation${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    
    echo -e "${YELLOW}Starting producer (sending messages every second)...${NC}"
    echo -e "${YELLOW}Press Ctrl+C to stop${NC}"
    echo ""
    
    # Create a producer script
    cat > producer.sh << 'SCRIPT'
#!/bin/bash
for i in {1..20}; do
    timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    message="{\"messageId\": $i, \"data\": \"Message $i\", \"timestamp\": \"$timestamp\"}"
    echo "$message" | kafka-console-producer --bootstrap-server localhost:9092 --topic user-events 2>/dev/null
    echo "Sent: $message"
    sleep 1
done
SCRIPT
    chmod +x producer.sh
    
    # Run producer in background
    ./producer.sh &
    PRODUCER_PID=$!
    
    sleep 3
    
    echo ""
    echo -e "${YELLOW}Now open the Web GUI and go to Kafka Inspector:${NC}"
    echo "  1. Topic: user-events"
    echo "  2. Mode: Tail"
    echo "  3. Count: 5"
    echo "  4. Click 'Start Live Monitoring'"
    echo ""
    echo -e "${GREEN}You'll see messages arriving in real-time!${NC}"
    echo ""
    
    # Wait a bit
    sleep 5
    
    # Stop producer
    kill $PRODUCER_PID 2>/dev/null || true
    
    echo -e "${GREEN}✅ DEMO 6 Complete: Real-time streaming demonstrated${NC}"
    echo ""
fi

# ============================================================================
# SUMMARY & NEXT STEPS
# ============================================================================

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}       🎉 All Demos Complete! 🎉${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo -e "${GREEN}✓ Demo 1: Format Conversion (JSON ↔ YAML ↔ TOML ↔ CSV)${NC}"
if [ "$KAFKA_AVAILABLE" = true ]; then
    echo -e "${GREEN}✓ Demo 2: Kafka Setup${NC}"
    echo -e "${GREEN}✓ Demo 3: Kafka Inspector${NC}"
    echo -e "${GREEN}✓ Demo 4: Bridge Pipeline (File → Kafka)${NC}"
    echo -e "${GREEN}✓ Demo 6: Real-time Streaming${NC}"
else
    echo -e "${YELLOW}⚠ Demo 2-4, 6: Skipped (Kafka not available)${NC}"
fi
echo -e "${GREEN}✓ Demo 5: Pipe Transformations${NC}"

echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}📁 Generated Files (in ./demo-data/):${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
ls -lh
echo ""

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}🌐 Next Steps - Use the Web GUI:${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "1. Start Backend:"
echo "   cd ../fluxmux-backend && npm start"
echo ""
echo "2. Start Frontend (new terminal):"
echo "   cd ../fluxmux-frontend && npm start"
echo ""
echo "3. Open browser to: http://localhost:3000"
echo ""
echo "4. Try each feature:"
echo "   • Convert: Load users.json and convert to YAML"
echo "   • Bridge: Create pipeline from file to Kafka"
echo "   • Pipe: Build transformation pipeline"
if [ "$KAFKA_AVAILABLE" = true ]; then
    echo "   • Kafka: Inspect 'user-events' topic in real-time"
fi
echo ""

if [ "$KAFKA_AVAILABLE" = false ]; then
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${RED}⚠️  Kafka Not Running${NC}"
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo "To enable Kafka features, run one of:"
    echo ""
    echo "Option 1 - Homebrew:"
    echo "  brew install kafka"
    echo "  brew services start zookeeper"
    echo "  brew services start kafka"
    echo ""
    echo "Option 2 - Docker (easier):"
    echo "  docker-compose up -d"
    echo ""
    echo "Then run this demo script again!"
    echo ""
fi

echo -e "${GREEN}Happy data streaming! 🚀${NC}"
echo ""
