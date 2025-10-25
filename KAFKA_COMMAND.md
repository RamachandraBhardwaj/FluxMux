# Kafka Inspection Command

The `kafka` command provides fast, real-time inspection of Kafka topics with minimal latency.

## Usage

```bash
fluxmux kafka --topic <topic-name> [options] <--head N | --tail N>
```

## Options

- `--topic <name>` - **Required**. Kafka topic to inspect
- `--broker <address>` - Kafka broker address (default: `localhost:9092`)
- `--group <id>` - Consumer group ID (default: `fluxmux-inspector`)
- `--head <N>` - Show first N messages from the topic (mutually exclusive with `--tail`)
- `--tail <N>` - Show latest N messages with live monitoring (mutually exclusive with `--head`)

## Head Mode

Shows the first N messages from the beginning of the topic.

**Features:**
- Displays N placeholder lines with "EOF" for empty slots
- Replaces EOF markers ASAP as messages arrive
- Exits automatically once N messages have been received
- Minimal latency with optimized consumer settings

**Example:**
```bash
# Show first 10 messages
fluxmux kafka --topic orders --head 10

# Output:
EOF
EOF
{"id":1,"product":"Widget","qty":5}
{"id":2,"product":"Gadget","qty":3}
EOF
EOF
EOF
EOF
EOF
EOF
```

**Use Cases:**
- Quick peek at topic contents
- Sample first messages for schema inspection
- Debug producer output
- Verify message format at topic start

## Tail Mode

Shows the latest N messages with continuous live monitoring.

**Features:**
- Displays real-time sliding window of the last N messages
- Updates instantly as new messages arrive (<100ms latency)
- Ring buffer maintains most recent N messages
- Clean terminal UI with message numbering
- Continues until Ctrl+C pressed

**Example:**
```bash
# Monitor last 5 messages
fluxmux kafka --topic logs --tail 5

# Output:
📡 Monitoring topic 'logs' - showing last 5 messages (Press Ctrl+C to exit)

[1] {"level":"INFO","msg":"User login","ts":"2024-01-01T10:00:00Z"}
[2] {"level":"WARN","msg":"High memory usage","ts":"2024-01-01T10:00:05Z"}
[3] {"level":"INFO","msg":"Request processed","ts":"2024-01-01T10:00:10Z"}
[4] {"level":"ERROR","msg":"DB connection failed","ts":"2024-01-01T10:00:15Z"}
[5] {"level":"INFO","msg":"Retry successful","ts":"2024-01-01T10:00:20Z"}
```

**Use Cases:**
- Live monitoring of production topics
- Real-time log inspection
- Debug live message flow
- Monitor event streams

## Performance Optimizations

Both modes are optimized for minimal latency:

### Consumer Configuration
- `fetch.min.bytes=1` - Don't wait for batching
- `fetch.wait.max.ms=50-100` - Short poll intervals
- `enable.auto.commit=true` - Automatic offset management
- `auto.offset.reset=earliest/latest` - Based on head/tail mode

### Terminal Control
- ANSI escape sequences for cursor positioning
- Hidden cursor during updates for clean display
- Minimal redraws (only changed lines)
- Buffered output for smooth rendering

## Comparison with Traditional Tools

### vs `kafka-console-consumer.sh`
```bash
# Traditional (verbose, no live updates)
kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic logs --from-beginning --max-messages 10

# FluxMux (clean, fast, EOF placeholders)
fluxmux kafka --topic logs --head 10
```

### vs `kafkacat/kcat`
```bash
# kafkacat (basic tail)
kafkacat -C -b localhost:9092 -t logs -o end -c 5

# FluxMux (live updates, numbered, clean UI)
fluxmux kafka --topic logs --tail 5
```

## Technical Details

### Head Mode Implementation
1. Subscribe to topic with `earliest` offset
2. Display N lines with "EOF" placeholders
3. Poll for messages with 100ms timeout
4. Replace EOF markers as messages arrive
5. Exit when N messages received

### Tail Mode Implementation
1. Subscribe to topic with `latest` offset
2. Initialize ring buffer with capacity N
3. Poll continuously with 50ms timeout
4. Add new messages to buffer (FIFO)
5. Redraw entire display on each update
6. Continue until interrupted

### Terminal Control Sequences
- `\x1B[2J` - Clear screen
- `\x1B[H` - Move cursor to home (0,0)
- `\x1B[?25l` - Hide cursor
- `\x1B[?25h` - Show cursor

## Examples

### Sample First 20 Messages
```bash
fluxmux kafka --topic user-events --broker prod-kafka:9092 --head 20
```

### Monitor Latest 10 Messages Live
```bash
fluxmux kafka --topic errors --broker prod-kafka:9092 --tail 10 --group error-monitor
```

### Quick Topic Inspection
```bash
# See what's in the topic
fluxmux kafka --topic my-topic --head 5

# Monitor live traffic
fluxmux kafka --topic my-topic --tail 3
```

## Limitations

1. **Text Only**: Optimized for JSON/text messages. Binary formats will display raw bytes.
2. **No Filtering**: Shows all messages. Use `pipe` command for filtering:
   ```bash
   fluxmux pipe kafka://localhost:9092/logs filter 'level==ERROR' -
   ```
3. **Single Topic**: Inspects one topic at a time.
4. **No Offset Control**: Head starts from beginning, tail from latest.

## Error Handling

### Common Errors

**No --head or --tail specified:**
```bash
$ fluxmux kafka --topic my-topic
Error: Either --head or --tail must be specified
```

**Connection failed:**
```bash
$ fluxmux kafka --topic my-topic --broker invalid:9092 --head 10
Kafka error: Failed to connect to broker
```

**Topic doesn't exist:**
```bash
$ fluxmux kafka --topic nonexistent --head 10
Kafka error: Unknown topic or partition
```

## Integration with Other Commands

The `kafka` command complements the existing `bridge` and `pipe` commands:

### Bridge: Full Pipeline
```bash
# Bridge for continuous streaming
fluxmux bridge --source kafka://localhost/input --sink file:output.json
```

### Pipe: Transformation
```bash
# Pipe for filtering and transformation
fluxmux pipe kafka://localhost/events filter 'temp>30' transform 'f=temp*1.8+32' -
```

### Kafka: Quick Inspection
```bash
# Kafka for rapid topic inspection
fluxmux kafka --topic events --head 10
```

## Future Enhancements

Potential improvements for future versions:

1. **Offset Control**: `--from-offset`, `--from-timestamp`
2. **Format Options**: `--format json|raw|pretty`
3. **Key Display**: Show message keys alongside values
4. **Partition Control**: `--partition N`
5. **Header Display**: Show Kafka message headers
6. **Color Coding**: Syntax highlighting for JSON
7. **Search/Filter**: `--grep`, `--match`

## See Also

- [Bridge Command](BRIDGE_IMPLEMENTATION.md) - Full streaming pipelines
- [Pipe Command](PIPE_COMMAND.md) - Unix-style data transformation
- [README](README.md) - Complete FluxMux overview
