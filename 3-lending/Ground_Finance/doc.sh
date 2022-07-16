#!/bin/bash

set -x
set -e

cd "$(dirname "$0")"

cd Ground_Test;
cargo doc --no-deps  --document-private-items --package ground_finance --package ground_test --open;
