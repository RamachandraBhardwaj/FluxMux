Write-Host "=== Testing kafka tail with in-place updates ===" -ForegroundColor Cyan
Write-Host "This demonstrates cursor movement for in-place overwriting`n" -ForegroundColor Gray

Write-Host "Starting interactive FluxMux..." -ForegroundColor Yellow
Write-Host "Try: kafka --topic my-test-topic --tail 5" -ForegroundColor Gray
Write-Host "Then from another terminal send messages to that topic" -ForegroundColor Gray
Write-Host "Watch as numbers update in-place (not reprinting above)`n" -ForegroundColor Gray

.\target\debug\fluxmux-cli
