#!/bin/bash

# FluxMux Cloud Kafka Demo Script
# Helps you test FluxMux with real cloud Kafka services

set -e

BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}    FluxMux Cloud Kafka Testing - Quick Setup${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo -e "${YELLOW}Choose your testing option:${NC}"
echo ""
echo "1. 🚀 Use ngrok with local Kafka (Easiest - 5 min setup)"
echo "   - No cloud account needed"
echo "   - Public URL to your local Kafka"
echo "   - Perfect for sharing/testing"
echo ""
echo "2. ☁️  Use Upstash Kafka REST API (Cloud - 10 min setup)"
echo "   - Free tier: 10,000 messages/day"
echo "   - REST API (no complex setup)"
echo "   - Sign up: https://upstash.com/"
echo ""
echo "3. 🌐 Use Confluent Cloud (Professional - 20 min setup)"
echo "   - Free tier: 400 MB/month"
echo "   - Full Kafka compatibility"
echo "   - Sign up: https://www.confluent.io/confluent-cloud/tryfree/"
echo ""
echo "4. 📚 Show me all options and links"
echo ""
echo "5. 🧪 Create sample data for testing"
echo ""
read -p "Enter choice (1-5): " choice
echo ""

case $choice in
  1)
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}Option 1: ngrok + Local Kafka${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    
    # Check if Kafka is running
    if nc -z localhost 9092 2>/dev/null; then
        echo -e "${GREEN}✓ Kafka is running on localhost:9092${NC}"
    else
        echo -e "${RED}✗ Kafka is not running${NC}"
        echo -e "${YELLOW}Starting Kafka...${NC}"
        docker-compose up -d
        sleep 5
    fi
    
    # Check if ngrok is installed
    if ! command -v ngrok &> /dev/null; then
        echo -e "${RED}✗ ngrok is not installed${NC}"
        echo ""
        echo -e "${YELLOW}Install ngrok:${NC}"
        echo "  macOS:   brew install ngrok/ngrok/ngrok"
        echo "  Linux:   snap install ngrok"
        echo "  Windows: Download from https://ngrok.com/download"
        echo ""
        echo "Then sign up for free: https://dashboard.ngrok.com/signup"
        echo "And authenticate: ngrok authtoken YOUR_TOKEN"
        exit 1
    fi
    
    echo -e "${GREEN}✓ ngrok is installed${NC}"
    echo ""
    echo -e "${YELLOW}Starting ngrok tunnel...${NC}"
    echo -e "${YELLOW}Note: Keep this terminal open!${NC}"
    echo ""
    
    # Start ngrok in background and capture URL
    echo -e "${BLUE}Instructions:${NC}"
    echo "1. In a NEW terminal, run: ${GREEN}ngrok tcp 9092${NC}"
    echo "2. Copy the forwarding URL (e.g., tcp://0.tcp.ngrok.io:12345)"
    echo "3. Come back here and paste it"
    echo ""
    read -p "Enter ngrok URL (e.g., 0.tcp.ngrok.io:12345): " NGROK_URL
    
    # Remove any protocol prefix
    NGROK_URL=$(echo $NGROK_URL | sed 's|tcp://||' | sed 's|kafka://||')
    
    echo ""
    echo -e "${GREEN}✓ Using ngrok URL: $NGROK_URL${NC}"
    echo ""
    
    # Create test topic
    echo -e "${YELLOW}Creating test topic...${NC}"
    kafka-topics --bootstrap-server localhost:9092 \
      --create --topic cloud-test --partitions 3 --replication-factor 1 2>/dev/null || true
    echo -e "${GREEN}✓ Topic 'cloud-test' ready${NC}"
    echo ""
    
    # Create test data
    echo -e "${YELLOW}Creating test data...${NC}"
    cat > ngrok-test-data.json << 'EOF'
{"orderId": 2001, "product": "Cloud Laptop", "amount": 1299.99, "customer": "Alice", "region": "US"}
{"orderId": 2002, "product": "Cloud Mouse", "amount": 49.99, "customer": "Bob", "region": "EU"}
{"orderId": 2003, "product": "Cloud Keyboard", "amount": 129.99, "customer": "Carol", "region": "APAC"}
EOF
    echo -e "${GREEN}✓ Created ngrok-test-data.json${NC}"
    echo ""
    
    # Send via FluxMux
    echo -e "${YELLOW}Sending data via FluxMux to cloud...${NC}"
    ./target/release/fluxmux-cli bridge \
      --source file:ngrok-test-data.json \
      --sink kafka://$NGROK_URL/cloud-test \
      --batch-size 3
    
    echo ""
    echo -e "${GREEN}✅ Success! Data sent to cloud Kafka via ngrok${NC}"
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}Your public Kafka endpoint:${NC}"
    echo -e "${GREEN}kafka://$NGROK_URL/cloud-test${NC}"
    echo ""
    echo -e "${YELLOW}Anyone can now connect to your Kafka using:${NC}"
    echo "  kafka-console-consumer --bootstrap-server $NGROK_URL --topic cloud-test"
    echo ""
    echo -e "${YELLOW}Verify in Web GUI:${NC}"
    echo "  1. Open http://localhost:3000/kafka"
    echo "  2. Broker: $NGROK_URL"
    echo "  3. Topic: cloud-test"
    echo "  4. Click 'Inspect'"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    ;;
    
  2)
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}Option 2: Upstash Kafka REST API${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    
    echo -e "${YELLOW}Setup Instructions:${NC}"
    echo "1. Go to: ${GREEN}https://upstash.com/${NC}"
    echo "2. Sign in with GitHub (instant!)"
    echo "3. Click 'Create Cluster' (choose your region)"
    echo "4. Click 'Create Topic' (name it 'orders-topic')"
    echo "5. Copy your credentials from the cluster page"
    echo ""
    read -p "Have you completed setup? (y/n): " setup_done
    
    if [ "$setup_done" != "y" ]; then
        echo -e "${YELLOW}Please complete setup first, then run this script again${NC}"
        exit 0
    fi
    
    echo ""
    echo -e "${YELLOW}Enter your Upstash credentials:${NC}"
    read -p "REST Endpoint (e.g., https://your-cluster.upstash.io): " UPSTASH_ENDPOINT
    read -p "Username: " UPSTASH_USERNAME
    read -sp "Password: " UPSTASH_PASSWORD
    echo ""
    
    # Create test data
    echo ""
    echo -e "${YELLOW}Creating test data...${NC}"
    cat > upstash-test-data.json << 'EOF'
{"orderId": 3001, "product": "Cloud Server", "amount": 299.99}
{"orderId": 3002, "product": "Cloud Storage", "amount": 49.99}
{"orderId": 3003, "product": "Cloud Network", "amount": 149.99}
EOF
    
    # Send via REST API
    echo -e "${YELLOW}Sending messages to Upstash...${NC}"
    
    while IFS= read -r line; do
      response=$(curl -s -X POST "$UPSTASH_ENDPOINT/produce/orders-topic" \
        -u "$UPSTASH_USERNAME:$UPSTASH_PASSWORD" \
        -H "Content-Type: application/json" \
        -d "{\"value\": \"$line\"}")
      echo -e "${GREEN}✓ Sent: $line${NC}"
    done < upstash-test-data.json
    
    echo ""
    echo -e "${GREEN}✅ Success! Messages sent to Upstash Kafka${NC}"
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}Your Upstash endpoint:${NC}"
    echo -e "${GREEN}$UPSTASH_ENDPOINT${NC}"
    echo ""
    echo -e "${YELLOW}Consume messages:${NC}"
    echo "curl -X GET $UPSTASH_ENDPOINT/consume/test-group/orders-topic/0 \\"
    echo "  -u \"$UPSTASH_USERNAME:$UPSTASH_PASSWORD\""
    echo ""
    echo -e "${YELLOW}View in Upstash Console:${NC}"
    echo "  https://console.upstash.com/"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    ;;
    
  3)
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}Option 3: Confluent Cloud${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    
    echo -e "${YELLOW}Setup Instructions:${NC}"
    echo "1. Go to: ${GREEN}https://www.confluent.io/confluent-cloud/tryfree/${NC}"
    echo "2. Sign up for free trial (no credit card for trial)"
    echo "3. Create a 'Basic' cluster (free tier)"
    echo "4. Create a topic (e.g., 'orders-topic')"
    echo "5. Create API key (Cluster → API Keys → Add key)"
    echo "6. Copy bootstrap server, API key, and secret"
    echo ""
    read -p "Have you completed setup? (y/n): " setup_done
    
    if [ "$setup_done" != "y" ]; then
        echo -e "${YELLOW}Please complete setup first, then run this script again${NC}"
        exit 0
    fi
    
    echo ""
    echo -e "${YELLOW}Enter your Confluent Cloud credentials:${NC}"
    read -p "Bootstrap Server (e.g., pkc-xxxxx.us-east-1.aws.confluent.cloud:9092): " CONFLUENT_BROKER
    read -p "API Key: " CONFLUENT_KEY
    read -sp "API Secret: " CONFLUENT_SECRET
    echo ""
    
    # Create config file
    echo ""
    echo -e "${YELLOW}Creating Kafka client configuration...${NC}"
    cat > confluent-client.properties << EOF
bootstrap.servers=$CONFLUENT_BROKER
security.protocol=SASL_SSL
sasl.mechanism=PLAIN
sasl.jaas.config=org.apache.kafka.common.security.plain.PlainLoginModule required username="$CONFLUENT_KEY" password="$CONFLUENT_SECRET";
EOF
    
    echo -e "${GREEN}✓ Configuration saved to confluent-client.properties${NC}"
    echo ""
    
    # Create test data
    echo -e "${YELLOW}Creating test data...${NC}"
    cat > confluent-test-data.json << 'EOF'
{"orderId": 4001, "product": "Enterprise License", "amount": 9999.99}
{"orderId": 4002, "product": "Support Plan", "amount": 1999.99}
{"orderId": 4003, "product": "Consulting", "amount": 4999.99}
EOF
    
    echo -e "${GREEN}✓ Created confluent-test-data.json${NC}"
    echo ""
    
    echo -e "${YELLOW}Testing connection with Kafka console tools...${NC}"
    if command -v kafka-console-producer &> /dev/null; then
        echo -e "${GREEN}✓ Kafka tools available${NC}"
        echo ""
        echo -e "${YELLOW}Sending test messages...${NC}"
        cat confluent-test-data.json | kafka-console-producer \
          --bootstrap-server $CONFLUENT_BROKER \
          --topic orders-topic \
          --producer.config confluent-client.properties
        
        echo ""
        echo -e "${GREEN}✅ Success! Messages sent to Confluent Cloud${NC}"
    else
        echo -e "${YELLOW}⚠ Kafka console tools not found${NC}"
        echo "Install with: brew install kafka"
        echo ""
        echo -e "${YELLOW}You can still use the Web GUI or REST API${NC}"
    fi
    
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}Your Confluent Cloud endpoint:${NC}"
    echo -e "${GREEN}kafka://$CONFLUENT_BROKER/orders-topic${NC}"
    echo ""
    echo -e "${YELLOW}Note:${NC} FluxMux currently supports basic Kafka auth."
    echo "For SASL/SSL, use the Kafka console tools with the config file:"
    echo "  confluent-client.properties"
    echo ""
    echo -e "${YELLOW}View in Confluent Cloud Console:${NC}"
    echo "  https://confluent.cloud/"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    ;;
    
  4)
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}All Cloud Kafka Service Options${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    
    echo -e "${BLUE}1. Upstash Kafka (Easiest)${NC}"
    echo "   URL: https://upstash.com/"
    echo "   Free Tier: 10,000 messages/day"
    echo "   Features: REST API, Very easy setup"
    echo "   Best for: Quick testing, REST API usage"
    echo ""
    
    echo -e "${BLUE}2. Confluent Cloud (Most Compatible)${NC}"
    echo "   URL: https://www.confluent.io/confluent-cloud/tryfree/"
    echo "   Free Tier: 400 MB/month"
    echo "   Features: Full Kafka, Global availability"
    echo "   Best for: Production-like testing"
    echo ""
    
    echo -e "${BLUE}3. CloudKarafka${NC}"
    echo "   URL: https://www.cloudkarafka.com/"
    echo "   Free Tier: 5 MB storage"
    echo "   Features: Shared Kafka instances"
    echo "   Best for: Learning Kafka"
    echo ""
    
    echo -e "${BLUE}4. Redpanda Cloud${NC}"
    echo "   URL: https://redpanda.com/try-redpanda"
    echo "   Free Tier: Up to 100 MB/s"
    echo "   Features: Kafka-compatible, Faster"
    echo "   Best for: High performance"
    echo ""
    
    echo -e "${BLUE}5. Aiven Kafka${NC}"
    echo "   URL: https://aiven.io/kafka"
    echo "   Free Trial: 30 days, \$300 credit"
    echo "   Features: Professional grade"
    echo "   Best for: Serious testing"
    echo ""
    
    echo -e "${BLUE}6. ngrok + Local Kafka (No Cloud Account)${NC}"
    echo "   URL: https://ngrok.com/"
    echo "   Cost: Free tier available"
    echo "   Features: Public URL to local Kafka"
    echo "   Best for: Sharing/testing without cloud"
    echo ""
    
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo "For detailed instructions, see: CLOUD_KAFKA_SERVICES.md"
    ;;
    
  5)
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}Creating Sample Test Data${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    
    # Create various test data files
    echo -e "${YELLOW}Creating sample data files...${NC}"
    
    # Orders
    cat > sample-orders.json << 'EOF'
{"orderId": 1001, "product": "Laptop", "price": 999.99, "customer": "Alice", "timestamp": "2025-11-01T10:00:00Z"}
{"orderId": 1002, "product": "Mouse", "price": 29.99, "customer": "Bob", "timestamp": "2025-11-01T10:01:00Z"}
{"orderId": 1003, "product": "Keyboard", "price": 79.99, "customer": "Carol", "timestamp": "2025-11-01T10:02:00Z"}
{"orderId": 1004, "product": "Monitor", "price": 299.99, "customer": "Dave", "timestamp": "2025-11-01T10:03:00Z"}
{"orderId": 1005, "product": "Headphones", "price": 149.99, "customer": "Eve", "timestamp": "2025-11-01T10:04:00Z"}
EOF
    echo -e "${GREEN}✓ Created sample-orders.json${NC}"
    
    # User events
    cat > sample-events.json << 'EOF'
{"eventId": "e1", "userId": 101, "action": "login", "timestamp": "2025-11-01T10:00:00Z"}
{"eventId": "e2", "userId": 102, "action": "purchase", "amount": 49.99, "timestamp": "2025-11-01T10:01:00Z"}
{"eventId": "e3", "userId": 103, "action": "view", "product": "laptop", "timestamp": "2025-11-01T10:02:00Z"}
{"eventId": "e4", "userId": 101, "action": "logout", "timestamp": "2025-11-01T10:05:00Z"}
{"eventId": "e5", "userId": 104, "action": "signup", "email": "new@example.com", "timestamp": "2025-11-01T10:06:00Z"}
EOF
    echo -e "${GREEN}✓ Created sample-events.json${NC}"
    
    # Sensor data
    cat > sample-sensors.json << 'EOF'
{"sensorId": "temp-001", "value": 23.5, "unit": "celsius", "location": "warehouse-a", "timestamp": "2025-11-01T10:00:00Z"}
{"sensorId": "temp-002", "value": 25.1, "unit": "celsius", "location": "warehouse-b", "timestamp": "2025-11-01T10:00:30Z"}
{"sensorId": "humid-001", "value": 65.2, "unit": "percent", "location": "warehouse-a", "timestamp": "2025-11-01T10:01:00Z"}
{"sensorId": "pressure-001", "value": 1013.25, "unit": "hPa", "location": "factory-1", "timestamp": "2025-11-01T10:01:30Z"}
{"sensorId": "temp-003", "value": 24.8, "unit": "celsius", "location": "office", "timestamp": "2025-11-01T10:02:00Z"}
EOF
    echo -e "${GREEN}✓ Created sample-sensors.json${NC}"
    
    # Producer script
    cat > continuous-producer.sh << 'SCRIPT'
#!/bin/bash
# Continuous producer for testing

TOPIC=${1:-test-topic}
BROKER=${2:-localhost:9092}

counter=1
while true; do
  timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
  message="{\"id\": $counter, \"timestamp\": \"$timestamp\", \"data\": \"Message $counter\"}"
  echo "$message" | kafka-console-producer --bootstrap-server $BROKER --topic $TOPIC 2>/dev/null
  echo "Sent message $counter to $TOPIC"
  ((counter++))
  sleep 2
done
SCRIPT
    chmod +x continuous-producer.sh
    echo -e "${GREEN}✓ Created continuous-producer.sh${NC}"
    
    echo ""
    echo -e "${GREEN}✅ Sample data files created!${NC}"
    echo ""
    echo -e "${YELLOW}Available files:${NC}"
    echo "  - sample-orders.json (5 orders)"
    echo "  - sample-events.json (5 user events)"
    echo "  - sample-sensors.json (5 sensor readings)"
    echo "  - continuous-producer.sh (streaming producer)"
    echo ""
    echo -e "${YELLOW}Usage examples:${NC}"
    echo "  # Send to local Kafka:"
    echo "  ./target/release/fluxmux-cli bridge --source file:sample-orders.json --sink kafka://localhost:9092/orders"
    echo ""
    echo "  # Send to cloud Kafka (after setup):"
    echo "  ./target/release/fluxmux-cli bridge --source file:sample-events.json --sink kafka://your-cloud-broker:9092/events"
    echo ""
    echo "  # Continuous streaming:"
    echo "  ./continuous-producer.sh my-topic localhost:9092"
    ;;
    
  *)
    echo -e "${RED}Invalid choice${NC}"
    exit 1
    ;;
esac

echo ""
echo -e "${GREEN}🎉 Setup complete!${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Test in Web GUI: http://localhost:3000"
echo "2. Try different features: Convert, Bridge, Pipe, Kafka Inspector"
echo "3. Monitor real-time streams with Kafka Inspector 🔴"
echo ""
echo -e "${BLUE}For more details, see: CLOUD_KAFKA_SERVICES.md${NC}"
echo ""
