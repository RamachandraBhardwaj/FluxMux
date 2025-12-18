# FluxMux Kafka with Variables - Complete Guide

## Key Rules

1. ✅ **DO NOT use backticks** with Windows file paths containing spaces
   - ❌ `bridge --source file:`${file}`` → ERROR (backticks break Windows paths)
   - ✅ `bridge --source file:${file}` → OK (correct)

2. ✅ **Use `${variable}` syntax** for variable substitution
   - ✅ `${file}`, `${mytopic}`, `${kafka-broker}`

3. ✅ **Never hardcode topics** - Always use variables for generic workflows

---

## Sending Data from File to Kafka

### Simple Bridge (Recommended)

```bash
# Set your variables
fluxmux> set file = input.json
[OK] Set file = input.json

fluxmux> set mytopic = my-topic
[OK] Set mytopic = my-topic

# Bridge data to Kafka (uses default broker: localhost:9092)
fluxmux> bridge --source file:${file} --sink kafka://localhost:9092/${mytopic}
[INFO] Starting bridge...
[OK] Bridge completed successfully
```

### With Custom Broker

```bash
fluxmux> set file = data.json
fluxmux> set broker = kafka-server:9092
fluxmux> set topic = processed-data

fluxmux> bridge --source file:${file} --sink kafka://${broker}/${topic}
[OK] Bridge completed successfully
```

### With Pipe (More Control)

```bash
# Send file to Kafka with filtering
fluxmux> pipe file:${file} filter 'age > 25' tee kafka://localhost:9092/${mytopic}
[OK] Pipe completed successfully

# Send with transformation
fluxmux> pipe file:${file} transform 'name=upper(name)' tee kafka://localhost:9092/${mytopic}
[OK] Pipe completed successfully

# Send with aggregation
fluxmux> pipe file:${file} aggregate --group-by region --sum amount tee kafka://localhost:9092/${mytopic}
[OK] Pipe completed successfully
```

---

## Reading from Kafka (Inspector)

### Dynamic Topic Selection

```bash
# Set topic as variable
fluxmux> set topic = my-topic
[OK] Set topic = my-topic

# Read first 10 messages
fluxmux> kafka --topic ${topic} --head 10
[Messages from Kafka...]

# Read last 5 messages
fluxmux> kafka --topic ${topic} --tail 5
[Messages from Kafka...]
```

### Using Custom Broker

```bash
fluxmux> set mytopic = vasudeva
fluxmux> set broker = kafka-server:9092

fluxmux> kafka --topic ${mytopic} --broker ${broker} --head 20
[Messages...]
```

### With Consumer Group

```bash
fluxmux> set topic = events
fluxmux> set group = my-consumer-group

fluxmux> kafka --topic ${topic} --group ${group} --tail 15
[Messages...]
```

---

## Complete Workflow Example

```bash
# Initialize session
fluxmux> set input-file = sales.json
[OK] Set input-file = sales.json

fluxmux> set output-topic = processed-sales
[OK] Set output-topic = processed-sales

fluxmux> set kafka-broker = localhost:9092
[OK] Set kafka-broker = localhost:9092

# Send to Kafka with transformation
fluxmux> pipe file:${input-file} \
  transform 'amount=amount*1.1' \
  tee kafka://${kafka-broker}/${output-topic}
[OK] Pipe completed successfully

# Verify data in Kafka
fluxmux> kafka --topic ${output-topic} --head 5
[5 records from Kafka...]

# View session state
fluxmux> status
Session Configuration
  file = <not set>
  kafka-broker = localhost:9092
  ...
Custom Variables
  input-file = sales.json
  output-topic = processed-sales
```

---

## Valid Syntax Patterns

| Pattern | Valid? | Example |
|---------|--------|---------|
| `${varname}` | ✅ Yes | `${file}`, `${mytopic}` |
| `$varname` | ✅ Yes | `$file`, `$mytopic` |
| `` `${varname}` `` | ❌ No | Don't use backticks with Windows paths |
| `{varname}` | ❌ No | Use `${varname}` not `{varname}` |
| hardcoded values | ✅ Yes | `kafka://localhost:9092/topic-name` |

---

## Common Issues & Solutions

### Issue: "Invalid Kafka URI format"
**Problem**: Wrong Kafka format
```
❌ kafka://vasudeva              → Missing host:port/topic
❌ localhost:9092/topic          → Missing kafka:// prefix
```
**Solution**:
```
✅ kafka://localhost:9092/topic  → Correct format
✅ kafka://${broker}/${topic}    → With variables
```

### Issue: File not found with spaces in path
**Problem**: Using backticks with Windows paths
```
❌ bridge --source file:`${file}`  → Backticks break path parsing
```
**Solution**:
```
✅ bridge --source file:${file}    → Without backticks
```

### Issue: Empty messages from Kafka
**Problem**: Topic doesn't have data or wrong topic name
```
fluxmux> kafka --topic wrong-topic --head 5
1) <nil>
2) <nil>
```
**Solution**: Verify topic name
```
fluxmux> set mytopic = correct-topic-name
fluxmux> kafka --topic ${mytopic} --head 5
[Your messages...]
```

---

## Command Reference

### Session Variables (Predefined)
- `${file}` or `$file` - Input file path
- `${sink}` or `$sink` - Output sink path
- `${kafka-broker}` - Kafka broker (default: localhost:9092)
- `${kafka-group}` - Consumer group (default: fluxmux-interactive)

### Custom Variables
Any variable you create: `set mycustom = value`
Then use: `${mycustom}`

### Commands Using Variables
- **bridge**: `--source` and `--sink` support variables
- **pipe**: All arguments support variables
- **kafka**: `--topic`, `--broker`, `--group` support variables
- **convert**: Input/output file paths support variables

---

## Best Practices

1. **Always use variables for dynamic values**
   ```bash
   ✅ set topic = ${mytopic}
   ✅ kafka --topic ${topic} --head 10
   ```

2. **Set all configuration once**
   ```bash
   set file = input.json
   set topic = my-data
   set broker = localhost:9092
   status  # Verify everything is set
   ```

3. **Reuse variables across commands**
   ```bash
   # Same variables for all operations
   bridge --source file:${file} --sink kafka://${broker}/${topic}
   pipe file:${file} | kafka --topic ${topic}
   ```

4. **Check session state before running**
   ```bash
   fluxmux> status  # See all variables
   fluxmux> help kafka  # See command details
   ```
