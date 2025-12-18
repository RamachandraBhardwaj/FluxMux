# FluxMux Bridge & Kafka Command Fixes

## Issues Fixed

### 1. **Bridge File Not Found Error**
**Problem**: Bridge command was showing "The system cannot find the file specified" error when using relative paths like `file:input.json`

**Root Cause**: FileSource wasn't properly resolving relative paths, causing Windows file system errors

**Solution**: Enhanced FileSource to resolve relative paths to absolute paths before attempting file operations
- Added `resolve_path()` method that converts relative paths to absolute paths using `std::fs::canonicalize()`
- Improved error messages to show the actual resolved path being used
- File paths are now properly resolved regardless of current working directory

### 2. **Kafka Head/Tail Display Glitching**
**Problem**: `kafka --head` and `kafka --tail` commands showed:
- Duplicate data being printed
- `<nil>` placeholder entries mixed with actual data
- Output glitching and cursor positioning issues
- Terminal display corruption with overlapping text

**Root Cause**: ANSI escape codes (`\x1B[H`, `\x1B[2K`) used for in-place screen updates don't work reliably on Windows console, especially when the terminal buffer overflows

**Solution**: Completely rewrote kafka_inspector to use simple sequential output
- **kafka_head**: Changed from in-place rendering to streaming output - messages printed as they arrive with numbered output
- **kafka_tail**: Changed to use VecDeque for sliding window of last N messages - maintains proper ordering while avoiding display glitches
- Removed ANSI escape codes that were causing terminal corruption
- Output is now clean, sequential, and properly formatted

## Files Modified

### crates/fluxmux-connectors/src/file.rs
- Added `resolve_path()` method to convert relative paths to absolute
- Enhanced error handling with path resolution details
- Fixed "file not found" errors on Windows

### crates/fluxmux-cli/src/kafka_inspector.rs
- Removed complex ANSI escape sequence rendering
- **kafka_head**: Simplified to stream messages with numbered output
- **kafka_tail**: Implemented VecDeque-based sliding window for clean last-N display
- Removed unused imports (KafkaMessage, io, Write)
- Added VecDeque import

### crates/fluxmux-core/src/engine.rs
- Cleaned up unused variables (middleware_count, message_count)
- Simplified code without changing logic

## Test Results

✅ **Bridge Command**
```
[INFO] Starting bridge: file:input.json → kafka://localhost:9092/test-clean
[OK] Bridge completed successfully
```

✅ **Kafka Head Command** (clean output without glitches)
```
1) {"active":true,"id":1001,"orders":[...],"user":{"name":"Alice",...}}
2) {"active":false,"id":1002,"orders":[...],"user":{"name":"Bob",...}}
```

✅ **Variable Substitution Works**
```
set file=input.json
bridge --source file:${file} --sink kafka://localhost:9092/topic
```

## Benefits

1. **Reliable File Operations**: Relative paths now work correctly on Windows
2. **Clean Terminal Output**: No more glitching or corruption
3. **No Duplicate Data**: Kafka output is streamed cleanly without re-rendering issues
4. **Better Error Messages**: Users see actual resolved file paths in error messages
5. **Consistent Behavior**: Both bridge and kafka commands work together properly

## Build Status

✅ Clean compilation with no errors or warnings (except 2 unused dead code warnings that are unrelated to these fixes)
