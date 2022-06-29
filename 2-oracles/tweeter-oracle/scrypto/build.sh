#!/bin/bash

set -x
set -e

cd "$(dirname "$0")"

(cd tweeter-oracle; scrypto build; cp target/wasm32-unknown-unknown/release/tweeter_oracle.wasm ../../public;)