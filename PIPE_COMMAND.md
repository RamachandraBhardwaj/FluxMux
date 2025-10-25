# FluxMux Pipe Command - Unix-Style Pipeline Processing

## Overview
The `pipe` command provides Unix-like inline data transformation pipelines with built-in actions for filtering, transforming, aggregating, and validating JSON data streams.

## Syntax

```bash
fluxmux pipe <source> [actions...] [outputs...]
```

- **Source**: file:path, kafka://host/topic, stdin, or -
- **Actions**: filter, transform, aggregate, normalize, validate, limit, sample
- **Outputs**: tee <destination>... or final destination (stdout, file:path, kafka://host/topic)

## Built-in Actions

### filter '<expression>'
Keeps only messages matching the condition.

**Supported operators**: `>`, `<`, `>=`, `<=`, `==`, `!=`

```powershell
# Keep temperatures above 30
Get-Content sensors.json | cargo run -p fluxmux-cli -- pipe stdin filter 'temp>30'

# Keep specific devices
cargo run -p fluxmux-cli -- pipe file:data.json filter 'device==sensor01'
```

### transform '<field>=<expression>'
Adds or modifies fields. Supports basic math expressions.

```powershell
# Convert Celsius to Fahrenheit
Get-Content temps.json | cargo run -p fluxmux-cli -- pipe stdin transform 'fahrenheit=temp*1.8+32'

# Multiple transformations
cargo run -p fluxmux-cli -- pipe file:data.json transform 'double=value*2,half=value/2'
```

### aggregate [options]
Groups and aggregates messages. Must be followed by `--` to separate from pipe args.

**Options**:
- `--group-by <field>`: Group by field value
- `--avg <field>`: Calculate average
- `--sum <field>`: Calculate sum
- `--min <field>`: Find minimum
- `--max <field>`: Find maximum
- `--count`: Count records

```powershell
# Average temperature by device
cargo run -p fluxmux-cli -- pipe file:sensors.json -- aggregate --group-by device --avg temp

# Multiple aggregations
cargo run -p fluxmux-cli -- pipe stdin -- aggregate --group-by category --sum amount --count
```

### validate --schema <path>
Filters out messages that don't match the schema requirements.

```powershell
# Validate against schema
Get-Content data.json | cargo run -p fluxmux-cli -- pipe stdin -- validate --schema schema.json
```

### normalize --schema <path>
Keeps only fields defined in the schema, removing extras.

```powershell
# Normalize to schema
cargo run -p fluxmux-cli -- pipe file:messy.json -- normalize --schema clean_schema.json
```

### limit <n>
Passes only the first N messages.

```powershell
# Get first 100 records
cargo run -p fluxmux-cli -- pipe file:large.json limit 100
```

### sample <n>
Passes every Nth message (sampling rate).

```powershell
# Sample every 10th message
cargo run -p fluxmux-cli -- pipe file:data.json sample 10
```

### tee <destination>...
Sends data to multiple outputs simultaneously.

```powershell
# Output to both file and stdout
Get-Content data.json | cargo run -p fluxmux-cli -- pipe stdin filter 'temp>30' tee file:hot.json stdout

# Send to file and Kafka
cargo run -p fluxmux-cli -- pipe file:data.json tee file:backup.json kafka://localhost:9092/events
```

## Complete Examples

### Example 1: Filter, Transform, and Save
```powershell
# Filter high temperatures, convert to Fahrenheit, save to file
Get-Content sensors.json | `
  cargo run -p fluxmux-cli -- pipe stdin `
  filter 'temp>30' `
  transform 'fahrenheit=temp*1.8+32' `
  file:hot_temps.json
```

### Example 2: Pipeline with Multiple Actions
```powershell
# Validate, filter, limit, and output
cargo run -p fluxmux-cli -- pipe file:data.json `
  -- validate --schema schema.json `
  filter 'age>25' `
  limit 10 `
  stdout
```

### Example 3: Aggregation Pipeline
```powershell
# Group by category and calculate statistics
cargo run -p fluxmux-cli -- pipe file:sales.json `
  -- aggregate --group-by product --sum amount --avg price --count
```

### Example 4: Multi-Output with Tee
```powershell
# Filter and send to multiple destinations
Get-Content events.json | `
  cargo run -p fluxmux-cli -- pipe stdin `
  filter 'severity==critical' `
  tee file:alerts.json kafka://localhost:9092/alerts stdout
```

### Example 5: Real-time Kafka Processing
```powershell
# Read from Kafka, filter, transform, write to file
cargo run -p fluxmux-cli -- pipe kafka://localhost:9092/sensors?group=processor `
  filter 'temp>50' `
  transform 'alert=true,timestamp=now' `
  file:critical_alerts.json
```

## PowerShell Pipeline Integration

FluxMux pipe works seamlessly with PowerShell pipelines:

```powershell
# Pipe from file content
Get-Content input.json | cargo run -p fluxmux-cli -- pipe stdin filter 'value>100'

# Chain with other tools
Get-Content data.json | `
  cargo run -p fluxmux-cli -- pipe stdin transform 'double=value*2' | `
  ConvertFrom-Json | `
  Format-Table
```

## Tips

1. **Use `--` separator** when action options start with `--` (e.g., aggregate, validate, normalize)
2. **Default output** is stdout if no destination specified
3. **Expressions** support field references and basic math (`+`, `-`, `*`, `/`)
4. **Multiple tee destinations** for broadcasting data
5. **Combine with bridge** for Kafka/DB integrations

## Comparison: Bridge vs Pipe

| Feature | Bridge | Pipe |
|---------|--------|------|
| **Purpose** | Source → Middleware → Sink | Source → Actions → Output(s) |
| **Middleware** | Batch, throttle, retry, dedupe | Filter, transform, aggregate |
| **Configuration** | YAML + CLI flags | Inline actions |
| **Outputs** | Single sink | Multiple via tee |
| **Use Case** | Production data pipelines | Ad-hoc transformations |

## Status

✅ **Fully Implemented**
- filter, transform, limit, sample, validate, normalize actions
- tee for multiple outputs
- stdin/stdout pipe integration
- Expression evaluator for math and comparisons
- Aggregate action (basic - needs refinement for complex grouping)

🚧 **Future Enhancements**
- buffer action for caching
- More complex expression language
- Nested field access (e.g., `data.metrics.temperature`)
- Aggregate improvements for better grouping
