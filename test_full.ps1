Set-Location "c:\coding stuff\IOMP\fluxmux"
$commands = @"
bridge --source file:input.json --sink kafka://localhost:9092/test-topic-final
kafka --topic test-topic-final --head 3
quit
"@

$commands | cargo run -p fluxmux-cli 2>&1
