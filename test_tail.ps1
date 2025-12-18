# Start kafka tail in background and capture it
$messages = @(
    '{"test":1}',
    '{"test":2}',
    '{"test":3}',
    '{"test":4}',
    '{"test":5}'
)

Write-Host "Starting kafka tail..."
$tailJob = Start-Job -ScriptBlock {
    cd "c:\coding stuff\IOMP\fluxmux"
    & .\target\debug\fluxmux-cli.exe kafka --topic tailtest --tail 3
} 

Start-Sleep -Seconds 1

Write-Host "Sending messages..."
foreach ($msg in $messages) {
    Write-Host "Sending: $msg"
    # Use echo and pipe to bridge command
    $msg | & ".\target\debug\fluxmux-cli.exe" bridge --file input.json --topic tailtest --pipe stdin
    Start-Sleep -Milliseconds 500
}

Start-Sleep -Seconds 2
Stop-Job -Job $tailJob -ErrorAction SilentlyContinue
Write-Host "Done"
