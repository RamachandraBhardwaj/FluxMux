# ☁️ Cloud Kafka Services for Real-Time Testing

Complete guide to free and public Kafka services you can use to test FluxMux with real-time data.

---

## 🚀 Quick Start - Free Cloud Kafka Services

### 1. **Confluent Cloud** (Free Tier) ⭐ **RECOMMENDED**

**Free Tier:** Yes (400 MB/month free)  
**Sign Up:** https://www.confluent.io/confluent-cloud/tryfree/

**Features:**
- ✅ Fully managed Kafka
- ✅ Global availability
- ✅ Free tier includes 1 cluster
- ✅ REST API access
- ✅ Schema Registry
- ✅ No credit card for trial

**Setup Steps:**
```bash
# 1. Sign up at: https://www.confluent.io/confluent-cloud/tryfree/

# 2. Create a cluster (choose "Basic" for free tier)

# 3. Create API key (copy the key and secret)

# 4. Get bootstrap server (something like):
# pkc-xxxxx.us-east-1.aws.confluent.cloud:9092

# 5. Create topic:
# Use web console or CLI

# 6. Use with FluxMux:
./target/release/fluxmux-cli bridge \
  --source file:data.json \
  --sink kafka://pkc-xxxxx.us-east-1.aws.confluent.cloud:9092/my-topic

# Note: You'll need to configure SASL authentication
```

**Connection String Example:**
```
kafka://pkc-xxxxx.us-east-1.aws.confluent.cloud:9092/orders-topic
```

---

### 2. **Upstash Kafka** ⭐ **EASIEST**

**Free Tier:** Yes (10,000 messages/day)  
**Sign Up:** https://upstash.com/

**Features:**
- ✅ Serverless Kafka
- ✅ REST API (HTTP endpoints!)
- ✅ Very easy setup
- ✅ No complex configuration
- ✅ GitHub login

**Setup Steps:**
```bash
# 1. Sign up at: https://upstash.com/

# 2. Create Kafka cluster (click "Create Cluster")

# 3. Create topic (click "Create Topic")

# 4. Get connection details:
#    Endpoint: https://evident-walrus-12345-us1-kafka.upstash.io
#    Username: ZXZpZGVudC13YWxydXMt...
#    Password: YourPasswordHere...

# 5. Use REST API with FluxMux:
curl -X POST https://evident-walrus-12345-us1-kafka.upstash.io/produce/my-topic \
  -u "username:password" \
  -d '{"message": "Hello from FluxMux!"}'
```

**REST API Example:**
```bash
# Produce message
curl -X POST https://your-cluster.upstash.io/produce/orders-topic \
  -u "$UPSTASH_USERNAME:$UPSTASH_PASSWORD" \
  -H "Content-Type: application/json" \
  -d '{
    "value": "{\"orderId\": 1001, \"amount\": 99.99}"
  }'

# Consume messages
curl -X GET https://your-cluster.upstash.io/consume/consumer-group/orders-topic/0 \
  -u "$UPSTASH_USERNAME:$UPSTASH_PASSWORD"
```

---

### 3. **CloudKarafka** (Free Tier)

**Free Tier:** Yes (5 MB storage, shared cluster)  
**Sign Up:** https://www.cloudkarafka.com/

**Features:**
- ✅ Shared Kafka instances
- ✅ Multiple cloud providers
- ✅ Free developer plan
- ✅ SASL/TLS support

**Setup Steps:**
```bash
# 1. Sign up at: https://www.cloudkarafka.com/

# 2. Create instance (select "Developer Duck" free plan)

# 3. Note your connection details:
#    Brokers: moped-01.srvs.cloudkafka.com:9094
#    Username: your-instance-name
#    Password: your-password
#    Prefix: your-instance-name-

# 4. Topics MUST be prefixed with your instance name:
#    your-instance-name-orders (not just "orders")

# 5. Use with authentication
```

**Connection Example:**
```bash
# CloudKarafka requires SASL authentication
# Example broker: moped-01.srvs.cloudkafka.com:9094
# Topic must include prefix: your-instance-name-orders
```

---

### 4. **Aiven Kafka** (Free Trial)

**Free Trial:** 30 days, $300 credit  
**Sign Up:** https://aiven.io/kafka

**Features:**
- ✅ Professional grade
- ✅ Multi-cloud support
- ✅ Good free trial
- ✅ Easy to use

---

### 5. **Redpanda Cloud** (Free Tier)

**Free Tier:** Yes (up to 100 MB/s)  
**Sign Up:** https://redpanda.com/try-redpanda

**Features:**
- ✅ Kafka API compatible
- ✅ Faster than Kafka
- ✅ Generous free tier
- ✅ No ZooKeeper needed

---

## 🧪 Public Test Kafka Brokers (No Auth)

### **Conduktor Public Kafka** (For Testing Only)

**Note:** These are public test brokers - not for production!

```bash
# Public broker (may be rate-limited)
broker: kafka.conduktor.io:9092

# Use with FluxMux:
./target/release/fluxmux-cli kafka \
  --topic test-topic \
  --broker kafka.conduktor.io:9092 \
  --tail 10
```

**Warning:** Public brokers may:
- Have rate limits
- Be unreliable
- Have data visible to others
- Be temporary/experimental

---

## 🎯 Complete Working Example with Upstash

Let me create a complete working example with Upstash (easiest option):

### Step 1: Sign Up and Setup

1. **Go to:** https://upstash.com/
2. **Sign in** with GitHub (instant!)
3. **Create Kafka Cluster:**
   - Click "Create Cluster"
   - Choose region (e.g., US East)
   - Click "Create"
4. **Create Topic:**
   - Click "Create Topic"
   - Name: `orders-topic`
   - Partitions: 3
   - Click "Create"
5. **Get Credentials:**
   - Click on your cluster
   - Copy REST endpoint
   - Copy username
   - Copy password

### Step 2: Test with curl

```bash
# Set your credentials
export UPSTASH_ENDPOINT="https://your-cluster.upstash.io"
export UPSTASH_USERNAME="your-username"
export UPSTASH_PASSWORD="your-password"

# Produce a test message
curl -X POST $UPSTASH_ENDPOINT/produce/orders-topic \
  -u "$UPSTASH_USERNAME:$UPSTASH_PASSWORD" \
  -H "Content-Type: application/json" \
  -d '{
    "value": "{\"orderId\": 1001, \"product\": \"Laptop\", \"amount\": 999.99}"
  }'

# Consume messages
curl -X GET $UPSTASH_ENDPOINT/consume/test-group/orders-topic/0 \
  -u "$UPSTASH_USERNAME:$UPSTASH_PASSWORD"
```

### Step 3: Create FluxMux Adapter Script

Since Upstash uses REST API, let's create a bridge script:

```bash
cat > upstash-producer.sh << 'SCRIPT'
#!/bin/bash

# Configuration
UPSTASH_ENDPOINT="https://your-cluster.upstash.io"
UPSTASH_USERNAME="your-username"
UPSTASH_PASSWORD="your-password"
TOPIC="orders-topic"

# Read from stdin or file and send to Upstash
while IFS= read -r line; do
  curl -s -X POST "$UPSTASH_ENDPOINT/produce/$TOPIC" \
    -u "$UPSTASH_USERNAME:$UPSTASH_PASSWORD" \
    -H "Content-Type: application/json" \
    -d "{\"value\": \"$line\"}"
  echo "Sent: $line"
done
SCRIPT

chmod +x upstash-producer.sh

# Use it:
cat data.json | ./upstash-producer.sh
```

---

## 🌐 Complete Real-Time Testing Workflow

### Option 1: Using Confluent Cloud (Most Kafka-Compatible)

```bash
# 1. Sign up: https://www.confluent.io/confluent-cloud/tryfree/

# 2. After setup, you'll get something like:
BROKER="pkc-xxxxx.us-east-1.aws.confluent.cloud:9092"
API_KEY="your-api-key"
API_SECRET="your-api-secret"
TOPIC="orders-topic"

# 3. Configure authentication (create config file)
cat > kafka-client.properties << EOF
bootstrap.servers=$BROKER
security.protocol=SASL_SSL
sasl.mechanism=PLAIN
sasl.jaas.config=org.apache.kafka.common.security.plain.PlainLoginModule required username="$API_KEY" password="$API_SECRET";
EOF

# 4. Test with kafka-console-producer (if you have Kafka CLI installed)
kafka-console-producer \
  --bootstrap-server $BROKER \
  --topic $TOPIC \
  --producer.config kafka-client.properties

# 5. For FluxMux, you'll need to add SASL support
# (currently FluxMux may need updates for SASL auth)
```

---

### Option 2: Using Local Kafka with ngrok (Best for Testing)

**This is the EASIEST way to test with real-time public access:**

```bash
# 1. Start local Kafka
docker-compose up -d

# 2. Install ngrok: https://ngrok.com/download
# Sign up for free account

# 3. Expose Kafka to internet
ngrok tcp 9092

# You'll get a public URL like:
# tcp://0.tcp.ngrok.io:12345

# 4. Now you can access from anywhere:
kafka-console-producer \
  --bootstrap-server 0.tcp.ngrok.io:12345 \
  --topic orders-topic

# 5. Use with FluxMux:
./target/release/fluxmux-cli bridge \
  --source file:data.json \
  --sink kafka://0.tcp.ngrok.io:12345/orders-topic
```

**ngrok Setup:**
```bash
# 1. Download: https://ngrok.com/download
# 2. Install:
brew install ngrok/ngrok/ngrok  # macOS

# 3. Sign up and get auth token: https://dashboard.ngrok.com/get-started/your-authtoken
# 4. Configure:
ngrok authtoken YOUR_AUTH_TOKEN

# 5. Expose Kafka:
ngrok tcp 9092

# Keep this terminal open!
```

---

## 🎬 Complete Demo with Public Kafka

### Demo Setup Script

```bash
cat > cloud-kafka-demo.sh << 'SCRIPT'
#!/bin/bash

echo "🌐 FluxMux Cloud Kafka Demo"
echo ""
echo "Choose your option:"
echo "1. Use ngrok with local Kafka (Easiest)"
echo "2. Use Upstash REST API"
echo "3. Use Confluent Cloud"
echo ""
read -p "Enter choice (1-3): " choice

case $choice in
  1)
    echo "🚀 Starting ngrok tunnel..."
    echo "1. Make sure Kafka is running: docker-compose up -d"
    echo "2. In another terminal, run: ngrok tcp 9092"
    echo "3. Copy the ngrok URL (e.g., 0.tcp.ngrok.io:12345)"
    read -p "Enter ngrok URL: " NGROK_URL
    
    echo "Creating test data..."
    echo '{"orderId": 1001, "amount": 99.99}' > test-order.json
    
    echo "Sending to Kafka via ngrok..."
    ./target/release/fluxmux-cli bridge \
      --source file:test-order.json \
      --sink kafka://$NGROK_URL/orders-topic
    
    echo "✅ Done! Check your Kafka consumer"
    ;;
    
  2)
    echo "📡 Using Upstash REST API"
    read -p "Enter Upstash endpoint: " ENDPOINT
    read -p "Enter username: " USERNAME
    read -p "Enter password: " PASSWORD
    
    echo "Sending test message..."
    curl -X POST $ENDPOINT/produce/orders-topic \
      -u "$USERNAME:$PASSWORD" \
      -H "Content-Type: application/json" \
      -d '{"value": "{\"orderId\": 1001, \"amount\": 99.99}"}'
    
    echo ""
    echo "✅ Message sent! Check Upstash console"
    ;;
    
  3)
    echo "☁️ Using Confluent Cloud"
    echo "Note: You need to configure SASL authentication"
    read -p "Enter bootstrap server: " BROKER
    read -p "Enter API key: " API_KEY
    read -p "Enter API secret: " API_SECRET
    
    echo "This requires SASL configuration..."
    echo "See: https://docs.confluent.io/cloud/current/client-apps/config-client.html"
    ;;
    
  *)
    echo "Invalid choice"
    ;;
esac
SCRIPT

chmod +x cloud-kafka-demo.sh
./cloud-kafka-demo.sh
```

---

## 📝 Summary - Best Options

### For Immediate Testing (No Setup):
**Use ngrok + Local Kafka:**
1. Start local Kafka: `docker-compose up -d`
2. Expose with ngrok: `ngrok tcp 9092`
3. Use ngrok URL: `kafka://0.tcp.ngrok.io:12345/topic`

### For REST API (Easiest Cloud):
**Use Upstash:**
1. Sign up: https://upstash.com/
2. Create cluster (1 click)
3. Use REST endpoints
4. Free tier: 10,000 messages/day

### For Production-Like Testing:
**Use Confluent Cloud:**
1. Sign up: https://www.confluent.io/confluent-cloud/tryfree/
2. Free tier: 400 MB/month
3. Full Kafka compatibility
4. Requires SASL auth

---

## 🔗 Quick Links

| Service | Sign Up | Free Tier | Ease of Use |
|---------|---------|-----------|-------------|
| **Upstash** | https://upstash.com/ | 10K msg/day | ⭐⭐⭐⭐⭐ |
| **Confluent Cloud** | https://www.confluent.io/confluent-cloud/tryfree/ | 400 MB/month | ⭐⭐⭐⭐ |
| **CloudKarafka** | https://www.cloudkarafka.com/ | 5 MB storage | ⭐⭐⭐ |
| **Redpanda Cloud** | https://redpanda.com/try-redpanda | 100 MB/s | ⭐⭐⭐⭐ |
| **Aiven** | https://aiven.io/kafka | 30-day trial | ⭐⭐⭐ |
| **ngrok (local)** | https://ngrok.com/ | Free tier | ⭐⭐⭐⭐⭐ |

---

## 🎯 Recommended Approach

**For FluxMux testing, I recommend:**

1. **Start with ngrok + local Kafka** (5 minutes setup)
   ```bash
   docker-compose up -d
   ngrok tcp 9092
   # Use the ngrok URL with FluxMux
   ```

2. **Then try Upstash** (10 minutes setup)
   ```bash
   # Sign up, create cluster, use REST API
   # Perfect for testing without local Kafka
   ```

3. **For serious testing, use Confluent Cloud** (20 minutes setup)
   ```bash
   # Full Kafka features, production-ready
   # Note: FluxMux may need SASL auth updates
   ```

---

## 🚀 Next Steps

1. **Choose a service** from above
2. **Sign up** (all have free tiers)
3. **Get connection details**
4. **Test with FluxMux**
5. **Monitor in real-time** with Kafka Inspector

---

**Happy Cloud Streaming! ☁️✨**

**Last Updated:** November 1, 2025
