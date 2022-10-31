#!/usr/bin/env bash
TESTING=1 ./builder.sh
cargo test --features testing
