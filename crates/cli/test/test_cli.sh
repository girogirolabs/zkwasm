#!/usr/bin/env bash
set -euo pipefail

reset () {
    rm -f fibb.wasm
    rm -rf ./output/*.data
    rm -rf ./output/*.json
    rm -rf ./params/*.data
    rm -f ./*.json

    mkdir -p ./output
    mkdir -p ./params
}

compile() {
    cargo build --release
    asc ./fibb.ts -o ./fibb.wasm
}

invoke_cli () {
    binpath="../../../target/release/delphinus-cli"
    RUST_LOG=info RUST_BACKTRACE=1 $binpath --function zkmain --param ./params --output ./output --wasm $1 $2
}

reset
compile
invoke_cli ./fibb.wasm setup
invoke_cli ./fibb.wasm checksum
invoke_cli ./fibb.wasm "single-prove --public 144:i64 --private 12:i64"
invoke_cli ./fibb.wasm single-verify
