# kafka head/tail with <nil> Placeholder Implementation
## User's Requirements
1. ✅ Keep `<nil>` entries as placeholders when topic has fewer messages than requested
2. ✅ Run async loop until Ctrl+C is pressed
3. ✅ Data gets printed in-place replacing `<nil>` when messages arrive
4. ✅ Return to normal FluxMux TUI prompt after Ctrl+C

## Implementation Details

### kafka_head function
- **Initialization**: Creates Vec with `n` slots, all initialized to `None` (displays as `<nil>`)
- **Display**: Shows `1) <nil>`, `2) <nil>`, etc. initially
- **Async Loop**: Listens for messages from Kafka topic
- **Update Mechanism**: 
  - When message arrives, replaces the `None` at position `received` with `Some(message)`
  - Clears screen with `\x1B[2J\x1B[H` and redraws entire display
  - All `<nil>` placeholders stay visible until replaced by actual data
- **Exit Condition**: After receiving `n` messages, waits 1 second then returns to prompt
- **Ctrl+C**: Shows cursor and returns to FluxMux prompt

### kafka_tail function
- **Initialization**: Creates Vec with `n` slots, all initialized to `None`
- **Display**: Shows `1) <nil>`, `2) <nil>`, etc. initially
- **Circular Buffer**: 
  - `next_slot` tracks where to place next message
  - As messages arrive, they replace `<nil>` entries
  - Once all slots filled, oldest message gets replaced by newest
- **Update Mechanism**:
  - Each new message replaces a `<nil>` or pushes out oldest message
  - Screen cleared with `\x1B[2J\x1B[H` and display redrawn
  - Messages shown in chronological order (oldest to newest)
- **Async Loop**: Continues indefinitely until Ctrl+C
- **Ctrl+C**: Shows cursor and returns to FluxMux prompt

### render_display helper function
```rust
fn render_display(messages: &[Option<String>]) {
    for (i, msg) in messages.iter().enumerate() {
        match msg {
            Some(content) => println!("{}) {}", i + 1, content),
            None => println!("{}) <nil>", i + 1),
        }
    }
}
```
Displays numbered list with either actual message content or `<nil>` placeholder.

## Key Features
- ✅ In-place updates: `<nil>` replaced by actual data as it arrives
- ✅ No duplicates: Screen cleared before redraw prevents duplicate output
- ✅ Proper async: Uses `tokio::select!` for message listening + Ctrl+C handling
- ✅ Clean terminal: Cursor hidden during updates, shown on exit
- ✅ Windows compatible: ANSI codes `\x1B[2J\x1B[H` work on modern Windows Terminal/PowerShell

## Example Output

### kafka head 5 (2 messages available in topic)
```
1) {"active":true,"id":1001}
2) {"active":false,"id":1002}
3) <nil>
4) <nil>
5) <nil>
```
When 3rd message arrives:
```
1) {"active":true,"id":1001}
2) {"active":false,"id":1002}
3) {"active":true,"id":1003}
4) <nil>
5) <nil>
```

### kafka tail 5 (new messages arriving)
Initial state:
```
1) <nil>
2) <nil>
3) <nil>
4) <nil>
5) <nil>
```
After first message:
```
1) {"timestamp":"2025-01-01T12:00:01Z","event":"user_login"}
2) <nil>
3) <nil>
4) <nil>
5) <nil>
```
After 3 messages:
```
1) {"timestamp":"2025-01-01T12:00:01Z","event":"user_login"}
2) {"timestamp":"2025-01-01T12:00:02Z","event":"purchase"}
3) {"timestamp":"2025-01-01T12:00:03Z","event":"logout"}
4) <nil>
5) <nil>
```
After 6th message (circular buffer wraps, oldest replaced):
```
2) {"timestamp":"2025-01-01T12:00:02Z","event":"purchase"}
3) {"timestamp":"2025-01-01T12:00:03Z","event":"logout"}
4) {"timestamp":"2025-01-01T12:00:04Z","event":"login"}
5) {"timestamp":"2025-01-01T12:00:05Z","event":"purchase"}
1) {"timestamp":"2025-01-01T12:00:06Z","event":"logout"}
```

## Ctrl+C Behavior
- Catches signal with `tokio::signal::ctrl_c()`
- Shows cursor again with `\x1B[?25h`
- Returns `Ok(())` which exits the command
- User returns to FluxMux prompt to run other commands
