#!/bin/bash
TARGET="${CARGO_TARGET_DIR:-target}"
set -e
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT/near

cargo build --target wasm32-unknown-unknown --release
cp $TARGET/wasm32-unknown-unknown/release/tipping_contract.wasm ./res/
