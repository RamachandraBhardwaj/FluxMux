Set-Location "c:\coding stuff\IOMP\fluxmux"
$commands = @"
bridge --source file:input.json --sink kafka://localhost:9092/test-fixed-topic
quit
"@

$commands | cargo run -p fluxmux-cli 2>&1
