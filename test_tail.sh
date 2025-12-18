#!/bin/bash

# Create a new topic and send test data while tail listens
TOPIC="tail-test-$(date +%s)"

echo "Topic: $TOPIC"
echo "Starting tail in background..."

cd /c/coding\ stuff/IOMP/fluxmux

# Start tail in background
./target/debug/fluxmux-cli kafka --topic "$TOPIC" --tail 5 &
TAIL_PID=$!

sleep 1

echo "Sending messages..."

# Simulate sending multiple messages to the kafka topic
# For now, just wait to see if tail displays anything
for i in {1..10}; do
  echo "Message $i"
  sleep 0.5
done

sleep 1

# Kill tail
kill $TAIL_PID 2>/dev/null
