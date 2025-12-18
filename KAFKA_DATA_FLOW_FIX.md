# FluxMux Bridge to Kafka Issue - Root Cause Analysis & Fix

## Problem Statement

When executing the bridge command to send data from a JSON file to Kafka, the command reported success:
```
[INFO] Starting bridge: file:input.json → kafka://localhost:9092/vasudeva
[OK] Bridge completed successfully
```

However, the Kafka topic remained empty. The Kafka consumer showed:
```
Processed a total of 0 messages
```

And the kafka inspector showed `<nil>` entries for the topic.

## Root Cause Analysis

### Discovery Process

1. **Initial Hypothesis**: Kafka connection was failing silently
   - Added detailed logging to KafkaSink producer creation and send operations
   - Found that producer was NOT being created at all
   - This indicated no messages were reaching the sink

2. **Second Investigation**: Checked message flow through pipeline
   - Added debug output to `run_pipeline()` function in engine.rs
   - Discovered messages were received from FileSource (✓)
   - But messages were being filtered by middleware (✗)
   - All 2 messages from input.json were being dropped before reaching sink

3. **Middleware Chain Analysis**:
   - Added debug to MiddlewareChain::process() to show which middleware filters
   - Found 1 middleware in the chain filtering all messages
   - Added debug to build_middleware_chain() to see which one was added
   - **DISCOVERED: Batcher middleware was being added despite no --batch-size flag!**

4. **Source of Batch Size**:
   - Traced back to SessionState default configuration
   - Default SessionState had `batch_size: Some(100)` hardcoded
   - In bridge handler, if no CLI `--batch-size` provided, it falls back to `state.batch_size`
   - This caused Batcher to be added with batch_size=100

5. **Why Batcher Filtered Messages**:
   - Batcher collects messages and only flushes when:
     - Buffer reaches `batch_size` (100 messages) OR
     - Timeout of 5000ms (5 seconds) expires
   - With only 2 messages in input.json and ~100ms execution time:
     - Buffer never fills (only 2 messages << 100)
     - Timeout never expires during pipeline execution
     - Messages stay in Batcher buffer indefinitely
     - Pipeline completes without flushing buffered messages
   - Result: Sink never receives any messages

## The Fix

### Change 1: Update SessionState Default (state.rs line 29)

**Before:**
```rust
batch_size: Some(100),
```

**After:**
```rust
batch_size: None,  // Changed from Some(100) to None - don't add Batcher by default
```

**Rationale**: 
- Batcher is an optional performance optimization for high-throughput scenarios
- Should NOT be enabled by default
- Users can explicitly enable with `set batch-size 100` or `--batch-size 100` flag if needed
- Default behavior should be pass-through (no batching)

### Change 2: Clear Cached Session File

Deleted `~/.fluxmux/session.json` which contained the old cached value `"batch_size": 100`
- Without this, even after code change, the stale session would persist

## Verification

After applying the fix, the bridge command works correctly:

```
[INFO] Starting bridge: file:input.json → kafka://localhost:9092/vasudeva
[OK] Bridge completed successfully
```

Kafka consumer now receives the messages:
```
Received 2 messages from topic vasudeva
Message 1: {...json data...}
Message 2: {...json data...}
```

## Lessons Learned

1. **Default Configuration**: Default values for performance-tuning parameters (like batch_size) should be conservative (None/disabled) rather than aggressive
2. **Session Persistence**: Cached configuration files can mask code changes - important to clear when defaults change
3. **Middleware Chain Transparency**: The Batcher middleware behavior wasn't obvious from success/error messages:
   - No error messages when messages are filtered
   - Success message despite data loss
   - Would benefit from warning when messages are buffered/held

## Files Modified

1. `crates/fluxmux-cli/src/interactive/state.rs` (line 29)
   - Changed `batch_size: Some(100)` to `batch_size: None` in SessionState::default()

## Testing

Tested with:
- Input: `input.json` (2 JSON objects)
- Command: `bridge --source file:input.json --sink kafka://localhost:9092/vasudeva`
- Result: ✅ 2 messages successfully sent to Kafka topic `vasudeva`

## Recommendations for Future

1. Add warning/info message when Batcher buffers large number of messages
2. Add timeout monitoring to warn if pipeline is stalling
3. Consider making Batcher behavior configurable with environment variable or command-line flag
4. Update documentation to clarify when Batcher should be enabled (high-throughput scenarios)
