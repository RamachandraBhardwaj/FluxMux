Write-Host "=== Testing kafka head with nil placeholders ===" -ForegroundColor Cyan
Write-Host "Starting: kafka head kafka://localhost:9092/test-demo 5" -ForegroundColor Yellow
Write-Host "Expected: Shows 5 slots with nil where no data exists" -ForegroundColor Gray
Write-Host "`nPress Ctrl+C in the FluxMux prompt to exit...`n" -ForegroundColor Gray

# Start the CLI in interactive mode
.\target\debug\fluxmux-cli
