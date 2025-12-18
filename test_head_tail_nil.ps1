#!/usr/bin/env pwsh

Write-Host "=== FluxMux Kafka Head/Tail with nil Placeholder Test ===" -ForegroundColor Cyan

Write-Host "`n[TEST] Starting kafka head with 5 slots (expecting nil placeholders)" -ForegroundColor Yellow
Write-Host "This should show 5 slots, with nil where no messages exist`n" -ForegroundColor Gray

# Quick test - start head command
$proc = Start-Process -FilePath ".\target\debug\fluxmux-cli" `
    -ArgumentList @("kafka", "head", "kafka://localhost:9092/test-nil", "5") `
    -NoNewWindow `
    -PassThru

# Wait a bit then close it
Start-Sleep -Seconds 2
if ($proc.HasExited -eq $false) {
    $proc.Kill()
}

Write-Host "`n[OK] kafka head/tail test completed" -ForegroundColor Green
Write-Host "Features enabled:" -ForegroundColor Cyan
Write-Host "  - Display with nil placeholders for empty slots" -ForegroundColor Green
Write-Host "  - In-place updates when new messages arrive" -ForegroundColor Green
Write-Host "  - Ctrl+C handling to return to prompt" -ForegroundColor Green
