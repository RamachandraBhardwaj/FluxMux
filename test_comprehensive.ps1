Set-Location "c:\coding stuff\IOMP\fluxmux"
$commands = @"
set file=input.json
set mytopic=comprehensive-test
bridge --source file:`${file} --sink kafka://localhost:9092/`${mytopic}
kafka --topic comprehensive-test --head 2
quit
"@

Write-Host "====== FluxMux Bridge & Kafka Comprehensive Test ======" -ForegroundColor Cyan
$commands | cargo run -p fluxmux-cli 2>&1 | Select-String "INFO|Starting|Bridge|OK|Set|alice|bob|active" | Where-Object {$_ -notmatch "warning|note"}
