# Kafka Command - Implementation Summary

## Overview
Implemented a fast Kafka topic inspection command with two modes: `--head` and `--tail`.

## Display Format

Both modes use a clean, numbered display:

```
1) <message content>
2) <message content>
3) <nil>
```

- Each slot is numbered: `1)`, `2)`, `3)`, etc.
- Messages appear on the same line as the number
- Empty slots show `<nil>` until filled
- Display is re-rendered in-place on each update (cursor moves to home, overwrites lines)
- No scrolling or appending - true in-place updates

## Head Mode

Shows the first N messages from the beginning of a topic.

**Behavior:**
- Displays N numbered slots with `<nil>` placeholders initially
- As messages arrive, replaces `<nil>` with actual message content
- Each update clears screen and re-renders entire display
- Exits automatically after N messages received

**Example:**
```bash
cargo run -p fluxmux-cli -- kafka --topic orders --head 5
```

**Initial display:**
```
1) <nil>
2) <nil>
3) <nil>
4) <nil>
5) <nil>
```

**After 2 messages:**
```
1) {"id":1,"product":"Widget"}
2) {"id":2,"product":"Gadget"}
3) <nil>
4) <nil>
5) <nil>
```

## Tail Mode

Shows a sliding window of the latest N messages with live monitoring.

**Behavior:**
- Displays N numbered slots with `<nil>` placeholders initially
- As messages arrive, adds them to a circular buffer
- Maintains oldest-to-newest order in the display
- Continuously monitors and updates until Ctrl+C
- Each update clears screen and re-renders entire display

**Example:**
```bash
cargo run -p fluxmux-cli -- kafka --topic logs --tail 3
```

**Initial display:**
```
1) <nil>
2) <nil>
3) <nil>
```

**After 1 message:**
```
1) {"level":"INFO","msg":"User login"}
2) <nil>
3) <nil>
```

**After 4 messages (sliding window):**
```
1) {"level":"WARN","msg":"High memory"}
2) {"level":"INFO","msg":"Request processed"}
3) {"level":"ERROR","msg":"DB connection failed"}
```

## Implementation Details

### Clean Rendering
- Uses `\x1B[H` to move cursor to home (0,0) for in-place updates
- Uses `\x1B[2K` to clear each line before writing
- Uses `\x1B[J` to clear from cursor to end of screen (removes leftover lines)
- Uses `\x1B[?25l` to hide cursor during updates
- Uses `\x1B[?25h` to show cursor on exit
- True in-place rendering - overwrites existing content without scrolling
- No appending to output stream

### Head Mode Logic
1. Subscribe to topic at `Offset::Beginning`
2. Initialize array of N `Option<String>` slots (all None)
3. Display initial numbered list with `<nil>` for each slot
4. On message arrival:
   - Store in next available slot
   - Clear screen and re-render all slots
5. Exit when N slots filled

### Tail Mode Logic
1. Subscribe to topic at `Offset::End` (latest offset)
2. Initialize array of N `Option<String>` slots (all None)
3. Track `next_slot` (circular index) and `total_received` count
4. Display initial numbered list with `<nil>` for each slot
5. On message arrival:
   - Store in current `next_slot` position
   - Increment `next_slot` circularly (% n)
   - Increment `total_received`
   - If less than N messages received: render slots in order
   - If N or more received: rearrange to show oldest→newest (from `next_slot`)
   - Clear screen and re-render all slots
6. Continue until Ctrl+C

### Kafka Configuration
**Head mode:**
- `auto.offset.reset=earliest`
- `enable.auto.commit=false`
- `fetch.min.bytes=1`
- `fetch.wait.max.ms=100`

**Tail mode:**
- `auto.offset.reset=latest`
- `enable.auto.commit=true`
- `fetch.min.bytes=1`
- `fetch.wait.max.ms=50`
- `session.timeout.ms=6000`
- `heartbeat.interval.ms=2000`

### Error Handling
- Graceful exit on Ctrl+C (shows cursor before exit)
- Displays Kafka errors with cursor visible
- Clean error messages for unknown topics/partitions

## Usage

```bash
# Head mode - show first 10 messages
cargo run -p fluxmux-cli -- kafka --topic my-topic --head 10

# Tail mode - monitor latest 5 messages
cargo run -p fluxmux-cli -- kafka --topic my-topic --tail 5

# With custom broker
cargo run -p fluxmux-cli -- kafka --topic my-topic --broker prod:9092 --tail 3

# With custom group
cargo run -p fluxmux-cli -- kafka --topic my-topic --group my-group --head 20
```

## Dependencies
- `rdkafka = { version = "0.36", features = ["tokio", "cmake-build"] }`
- `futures = "0.3"`
- Requires CMake in PATH (for cmake-build feature)

## Terminal Compatibility
- Works in Windows Terminal, PowerShell, VS Code terminal
- Requires ANSI escape sequence support
- Tested on Windows with PowerShell

## Performance
- Head mode: exits immediately after N messages
- Tail mode: <100ms latency for new messages
- Minimal overhead with optimized consumer settings
- Clean re-renders without flickering

## Testing Results
✅ Head mode: displays <nil>, replaces cleanly, exits after N
✅ Tail mode: displays <nil>, sliding window works, clean updates
✅ Ctrl+C handling: cursor restored properly
✅ Error handling: clean messages for invalid topics
✅ Display: no redundant output, clean numbered format
