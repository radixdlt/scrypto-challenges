#!/bin/bash

set -x
set -e

cd "$(dirname "$0")"

(cd mfa_oracle; scrypto build; cp target/wasm32-unknown-unknown/release/mfa_oracle.wasm ../../public)
