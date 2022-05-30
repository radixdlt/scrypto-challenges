#!/bin/bash

set -x
set -e

cd "$(dirname "$0")"

scrypto build
cp target/wasm32-unknown-unknown/release/callback_scheduler.wasm ../pte-demo/public