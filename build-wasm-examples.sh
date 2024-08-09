#!/bin/bash
# This script builds, optimizes and packages all the examples as WASM demos

set -ex

EXAMPLES=(
  "miner"
  "cells"
  "resman"
  "von_neumann"
)

for EXAMPLE in "${EXAMPLES[@]}"; do

  RUSTFLAGS="-Zlocation-detail=none" cargo build --profile=wasm-release --no-default-features --example="$EXAMPLE" --target=wasm32-unknown-unknown -Z build-std-features=panic_immediate_abort -Z build-std=std,panic_abort -Z build-std-features="optimize_for_size"

  time wasm-bindgen --out-name "$EXAMPLE" --no-typescript --out-dir web-src/wasm/examples --target web "target/wasm32-unknown-unknown/wasm-release/examples/$EXAMPLE.wasm"

  time wasm-opt -all -Oz -ol 10 -s 10 -o "web-src/wasm/examples/${EXAMPLE}_bg.wasm" "web-src/wasm/examples/${EXAMPLE}_bg.wasm" 

  time gzip -9 -c "web-src/wasm/examples/${EXAMPLE}_bg.wasm"  > "web-src/wasm/examples/${EXAMPLE}_bg.wasm.gz"

  time brotli -9 -c "web-src/wasm/examples/${EXAMPLE}_bg.wasm" > "web-src/wasm/examples/${EXAMPLE}_bg.wasm.b"

done

ls -hl web-src/wasm/examples/miner*
