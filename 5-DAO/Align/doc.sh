#!/bin/bash

set -x
set -e

cd "$(dirname "$0")"

cd scrypto;
cargo doc --no-deps  --document-private-items --open;