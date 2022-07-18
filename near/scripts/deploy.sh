#!/bin/bash

set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT/near/contracts/tipping

NEAR_ENV=testnet near state myriadtips.testnet

NEAR_ENV=testnet near deploy --accountId myriadtips.testnet --wasmFile ../../artifacts/tipping.wasm

NEAR_ENV=testnet near state myriadtips.testnet

NEAR_ENV=testnet near call myriadtips.testnet new '' --accountId b261c40ed3ebf00c9b2e738da015844491d2eaa2a29129b7f6df7743bd7161c1