#!/bin/bash

set -x
set -e

cd "$(dirname "$0")"

(cd auction; scrypto build; cp target/wasm32-unknown-unknown/release/auction.wasm ../../public)