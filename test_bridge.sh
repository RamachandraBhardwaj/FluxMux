#!/bin/bash
echo "bridge --source file:input.json --sink kafka://localhost:9092/vasudeva" | cargo run -p fluxmux-cli
