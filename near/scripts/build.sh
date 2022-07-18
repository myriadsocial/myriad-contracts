#!/bin/bash

set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT/near/contracts/tipping

cargo build --target wasm32-unknown-unknown --release

TARGET="${CARGO_TARGET_DIR:-target}"
cp $TARGET/wasm32-unknown-unknown/release/tipping.wasm ../../artifacts/
