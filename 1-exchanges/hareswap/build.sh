#!/usr/bin/env bash
set -e
# build both the Scrypto package and the hare CLI tool (this will take a while the first time)
scrypto build && (cd hare && cargo build)
