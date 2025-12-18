# Demonstration: Before vs After

## Before (BROKEN) - Reprinting Output

```
User: kafka --topic test --tail 3

Initial print (3 messages arrive):
1) {"id":1,"name":"Alice"}
2) {"id":2,"name":"Bob"}  
3) {"id":3,"name":"Charlie"}

Next message arrives (msg 4):
1) {"id":1,"name":"Alice"}          <- REPRINTED
2) {"id":2,"name":"Bob"}            <- REPRINTED
3) {"id":3,"name":"Charlie"}        <- REPRINTED
4) <nil>

Next message arrives (msg 5):
1) {"id":1,"name":"Alice"}          <- REPRINTED AGAIN
2) {"id":2,"name":"Bob"}            <- REPRINTED AGAIN
3) {"id":3,"name":"Charlie"}        <- REPRINTED AGAIN
4) {"id":4,"name":"Diana"}          <- REPRINTED
5) <nil>

Result: CLUTTERED SCREEN with duplicate output
```

---

## After (FIXED) - In-Place Updates

```
User: kafka --topic test --tail 5

Initial print (3 messages available):
1) {"id":1,"name":"Alice"}
2) {"id":2,"name":"Bob"}  
3) {"id":3,"name":"Charlie"}
4) <nil>
5) <nil>

Message 4 arrives:
[Cursor moves UP to line 4]
[Clears line 4]
[Prints updated line]

Display becomes:
1) {"id":1,"name":"Alice"}          <- UNCHANGED
2) {"id":2,"name":"Bob"}            <- UNCHANGED
3) {"id":3,"name":"Charlie"}        <- UNCHANGED
4) {"id":4,"name":"Diana"}          <- UPDATED (NEW)
5) <nil>                             <- UNCHANGED

Message 5 arrives:
[Cursor moves UP to line 5]
[Clears line 5]
[Prints updated line]

Display becomes:
1) {"id":1,"name":"Alice"}          <- UNCHANGED
2) {"id":2,"name":"Bob"}            <- UNCHANGED
3) {"id":3,"name":"Charlie"}        <- UNCHANGED
4) {"id":4,"name":"Diana"}          <- UNCHANGED
5) {"id":5,"name":"Eve"}            <- UPDATED (NEW)

Message 6 arrives (buffer full, circular wrap):
[Cursor moves UP 5 lines]
[Clears all 5 lines]
[Prints reordered buffer]

Display becomes:
1) {"id":2,"name":"Bob"}            <- UPDATED (oldest removed)
2) {"id":3,"name":"Charlie"}        <- SHIFTED
3) {"id":4,"name":"Diana"}          <- SHIFTED
4) {"id":5,"name":"Eve"}            <- SHIFTED
5) {"id":6,"name":"Frank"}          <- NEW (at end)

Result: CLEAN DISPLAY, no duplication, in-place overwrite
```

---

## How It Works (Technical)

### Problem: Using `\x1B[2J\x1B[H` (Clear Screen)
```
Attempt to clear entire screen in REPL:
┌─────────────────────────────┐
│ fluxmux> kafka --tail 3     │
│ 1) msg1                     │
│ 2) msg2                     │
│ 3) msg3                     │
│                             │  <- ANSI clears here
│                             │
│ 1) msg1                     │  <- But prompt stays above!
│ 2) msg2                     │     causes reprinting
│ 3) msg3                     │
│ 4) <nil>                    │
└─────────────────────────────┘
```

### Solution: Using `\x1B[A` + `\x1B[2K` (Move & Clear)
```
Move cursor up and clear specific lines:
┌─────────────────────────────┐
│ fluxmux> kafka --tail 3     │
│ 1) msg1                     │ <- Cursor here, print msg1
│ 2) msg2                     │ <- Cursor here, print msg2
│ 3) msg3                     │ <- Cursor here, print msg3
│ 4) <nil>                    │
│                             │
│ 5) <nil>                    │
│                             │
│ fluxmux> _                  │ <- REPL prompt stays put
└─────────────────────────────┘

New message arrives:
┌─────────────────────────────┐
│ fluxmux> kafka --tail 3     │
│ 1) msg1                     │
│ 2) msg2                     │
│ 3) msg3                     │
│ 4) <msg4>  ← Move up 1, clear, print  (ONLY THIS UPDATES)
│                             │
│ 5) <nil>                    │ <- Stays unchanged
│                             │
│ fluxmux> _                  │ <- REPL prompt stays put
└─────────────────────────────┘
```

---

## Algorithm (Pseudocode)

```rust
let mut first_print = true;

loop {
    let msg = receive_from_kafka();
    
    // Only move cursor on subsequent messages
    if !first_print {
        // Move up N lines (where N = window size)
        for i in 0..n {
            print!("\x1B[A");   // Move up 1 line
            print!("\x1B[2K");  // Clear it
        }
    }
    
    // Print all messages (overwrites the lines above)
    for (i, message) in messages.iter().enumerate() {
        println!("{}) {}", i+1, message_or_nil(message));
    }
    
    first_print = false;
}
```

---

## Edge Cases Handled

### 1. First Message (No Previous Output)
```
Before printing: just print normally, no cursor movement
After printing:  set first_print = false
```

### 2. Buffer Not Full
```
Circular buffer size: 5
Messages received: 3

Display:
1) msg1
2) msg2
3) msg3
4) <nil>
5) <nil>
```

### 3. Buffer Full (Circular Wrap)
```
Circular buffer size: 5
Messages received: 7

Buffer indices: [5, 6, 7, 3, 4]
Reorder for display:
1) msg3 (from index 2)
2) msg4 (from index 3)
3) msg5 (from index 4)
4) msg6 (from index 0)
5) msg7 (from index 1)
```

### 4. Ctrl+C Interrupt
```
User presses Ctrl+C at any time:
- tokio::signal::ctrl_c() triggers
- Function returns Ok(())
- Back to fluxmux> prompt
```

---

## Terminal Compatibility

| Terminal | Status | Notes |
|----------|--------|-------|
| Windows Terminal | ✅ | ANSI codes fully supported |
| PowerShell 7+ | ✅ | Works with Windows Terminal |
| Windows CMD | ⚠️ | Limited ANSI support |
| Linux Terminal | ✅ | Standard ANSI terminal |
| WSL | ✅ | Works like Linux |
| macOS Terminal | ✅ | Standard ANSI support |
| iTerm2 | ✅ | Full ANSI support |
