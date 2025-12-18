Set-Location "c:\coding stuff\IOMP\fluxmux"
$commands = @"
set file=input.json
set kafka-broker=localhost:9092
bridge --source file:`${file} --sink kafka://`${kafka-broker}/test-clean
kafka --topic test-clean --head 2
quit
"@

$commands | cargo run -p fluxmux-cli 2>&1
