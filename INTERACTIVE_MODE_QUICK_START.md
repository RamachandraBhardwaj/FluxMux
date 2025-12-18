# FluxMux Interactive Mode - Quick Start Guide

## Starting Interactive Mode

```bash
cd "c:\coding stuff\IOMP\fluxmux"
./target/debug/fluxmux-cli
```

You should see:
```
🚀 Welcome to FluxMux Interactive Mode
ℹ Type 'help' for available commands, 'exit' to quit

(fluxmux)> 
```

---

## Quick Command Reference

### Getting Help
```bash
(fluxmux)> help                 # Show all commands
(fluxmux)> help convert         # Show help for specific command
(fluxmux)> help status
```

### Managing Session Variables
```bash
(fluxmux)> status                           # View all session variables
(fluxmux)> set file input.json              # Set default input file
(fluxmux)> set sink output.csv              # Set default output file
(fluxmux)> set kafka-broker localhost:9092  # Set Kafka broker
(fluxmux)> set batch-size 500               # Set batch size
(fluxmux)> clear file                       # Reset specific variable
(fluxmux)> clear                            # Reset all variables
```

### File Conversion
```bash
(fluxmux)> convert input.csv output.json --from csv --to json
(fluxmux)> convert input.json output.yaml --from json --to yaml
(fluxmux)> convert input.toml output.json --from toml --to json
```

### Stream Processing
```bash
(fluxmux)> pipe file:input.json
(fluxmux)> pipe file:input.json filter 'id>1000' tee output.json
(fluxmux)> pipe file:input.csv transform 'name=upper(name)' tee output.csv
```

### Data Bridging (with Kafka)
```bash
(fluxmux)> bridge --source kafka://localhost:9092/mytopic --sink postgres://localhost/mydb
(fluxmux)> bridge --source file:input.json --sink kafka://localhost:9092/mytopic
```

### Kafka Inspection
```bash
(fluxmux)> kafka --topic mytopic --head 10
(fluxmux)> kafka --topic mytopic --tail 5 --broker localhost:9092
```

### Session Management
```bash
(fluxmux)> history              # Show last 20 commands
(fluxmux)> history 10           # Show last 10 commands
(fluxmux)> stats                # Show command usage statistics
(fluxmux)> quit                 # Exit and save session
(fluxmux)> exit                 # Alternative: exit and save session
# Or press Ctrl+D
```

---

## Session State Example

```bash
(fluxmux)> status
```

Output:
```
═══════════════════════════════════════════════════════════
              📋 FluxMux Session State
═══════════════════════════════════════════════════════════
  File Path:        input.json
  Sink Path:        output.csv
  Kafka Broker:     localhost:9092
  Kafka Group:      fluxmux-interactive
  Postgres Conn:    (not set)
  Batch Size:       500
  Deduplicate:      false
  Throttle/sec:     (not set)
═══════════════════════════════════════════════════════════
```

---

## Setting Variables for Convenience

Save commonly used settings:

```bash
(fluxmux)> set file input.json
(fluxmux)> set kafka-broker kafka.example.com:9092
(fluxmux)> set kafka-group mygroup
(fluxmux)> set batch-size 1000
(fluxmux)> set deduplicate true
(fluxmux)> status
```

These settings persist in `~/.fluxmux/session.json` and load automatically on next startup.

---

## Typical Workflow

### Session 1: Setup
```bash
$ ./target/debug/fluxmux-cli
(fluxmux)> set file mydata.json
(fluxmux)> set kafka-broker kafka.prod.com:9092
(fluxmux)> set batch-size 500
(fluxmux)> status
(fluxmux)> quit
```

### Session 2: Reuse Saved Settings
```bash
$ ./target/debug/fluxmux-cli
(fluxmux)> status                    # Shows previously saved settings
(fluxmux)> bridge --source kafka://kafka.prod.com:9092/raw --sink postgres://db/warehouse
(fluxmux)> history
(fluxmux)> stats
(fluxmux)> quit
```

---

## Color-Coded Output

| Color | Meaning | Example |
|-------|---------|---------|
| 🟢 Green | Success | `✓ Converted file successfully` |
| 🔴 Red | Error | `✗ File not found` |
| 🟡 Yellow | Warning | `⚠ Connection timeout` |
| 🔵 Blue | Info | `ℹ Session loaded` |
| 🔷 Cyan | Prompt | `(fluxmux)>` |

---

## File Locations

After using interactive mode, these files are created:

```
~/.fluxmux/
├── session.json    # Your saved session variables
└── history         # Command history (for readline)
```

### View Your Session File
```bash
cat ~/.fluxmux/session.json
```

Example output:
```json
{
  "file_path": "input.json",
  "sink_path": "output.csv",
  "kafka_broker": "localhost:9092",
  "kafka_group": "fluxmux-interactive",
  "postgres_conn": null,
  "batch_size": 500,
  "deduplicate": false,
  "throttle_per_sec": null
}
```

---

## Supported File Formats

| Format | Extensions | Example |
|--------|-----------|---------|
| CSV | .csv | `convert input.csv output.json --from csv --to json` |
| JSON | .json | `convert input.json output.yaml --from json --to yaml` |
| YAML | .yaml, .yml | `convert input.yaml output.csv --from yaml --to csv` |
| TOML | .toml | `convert input.toml output.json --from toml --to json` |

---

## Common Use Cases

### 1. Convert Multiple File Formats in One Session
```bash
(fluxmux)> set file data.csv
(fluxmux)> convert data.csv data.json --from csv --to json
(fluxmux)> set file data.json
(fluxmux)> convert data.json data.yaml --from json --to yaml
(fluxmux)> convert data.yaml data.toml --from yaml --to toml
```

### 2. Stream Processing with Filters
```bash
(fluxmux)> set file transactions.json
(fluxmux)> pipe file:transactions.json filter 'amount>1000' tee high_value.json
```

### 3. Kafka Pipeline Setup
```bash
(fluxmux)> set kafka-broker kafka.company.com:9092
(fluxmux)> set kafka-group analytics-team
(fluxmux)> kafka --topic events --head 50
(fluxmux)> bridge --source kafka://kafka.company.com:9092/raw-events --sink postgres://db/events_table
```

### 4. Batch Processing with Configuration
```bash
(fluxmux)> set file logs.json
(fluxmux)> set batch-size 5000
(fluxmux)> set deduplicate true
(fluxmux)> pipe file:logs.json filter 'level=ERROR' tee errors.log
```

---

## Tips & Tricks

### Use Tab Completion
Rustyline provides readline-style features:
- **Up/Down arrows**: Navigate command history
- **Ctrl+R**: Search command history
- **Ctrl+A/E**: Jump to start/end of line
- **Ctrl+K**: Clear to end of line

### Check What Changed
```bash
(fluxmux)> history 5          # See last 5 commands
(fluxmux)> stats              # See what you used most
```

### Persistent State Across Sessions
```bash
$ ./target/debug/fluxmux-cli
(fluxmux)> set file important.json
(fluxmux)> quit

$ ./target/debug/fluxmux-cli
(fluxmux)> status             # Shows file = important.json (loaded from disk!)
```

### Reuse Settings
```bash
(fluxmux)> set kafka-broker prod-kafka:9092
(fluxmux)> kafka --topic events --head 20      # Uses broker from settings
(fluxmux)> kafka --topic analytics --head 20   # Uses same broker
```

---

## Troubleshooting

### Session Not Persisting?
Check if `~/.fluxmux/session.json` exists:
```bash
ls ~/.fluxmux/session.json
```

If not found, the directory might not have been created. It will be created automatically on first use.

### Can't Remember Previous Commands?
```bash
(fluxmux)> history         # Shows all previous commands in session
```

Or use arrow keys to navigate readline history.

### Want to Start Fresh?
```bash
(fluxmux)> clear           # Resets all session variables
(fluxmux)> quit            # Saves cleared state
```

### File Not Found Errors?
Make sure you're using correct paths:
```bash
(fluxmux)> set file ./input.json      # Relative to current directory
(fluxmux)> set file /absolute/path/input.json  # Absolute path
```

---

## Performance Notes

- **Session Loading**: Instant (loads from JSON cache)
- **Small Files**: < 100MB - Process in memory
- **Large Files**: Use batch processing with `--batch-size`
- **Streaming**: Pipe command for large datasets
- **Throttling**: Use `set throttle-per-sec` to rate-limit processing

---

## Exiting Interactive Mode

Three ways to exit:

```bash
(fluxmux)> quit
# or
(fluxmux)> exit
# or press Ctrl+D
```

All three methods will:
1. ✓ Save session state to `~/.fluxmux/session.json`
2. ✓ Save command history to `~/.fluxmux/history`
3. ✓ Return to shell prompt with goodbye message
4. ✓ Exit with status 0

---

## Summary

| Action | Command | Result |
|--------|---------|--------|
| Start | `./fluxmux-cli` | Interactive prompt |
| Get Help | `help` | List all commands |
| View State | `status` | Current settings |
| Set Variable | `set file input.json` | Persistent setting |
| Run Command | `convert in.csv out.json --from csv --to json` | Execute immediately |
| View History | `history` | Last 20 commands |
| Show Stats | `stats` | Usage frequency |
| Exit | `quit` or `exit` | Save & return to shell |

---

**Interactive Mode is now ready for use! 🚀**
