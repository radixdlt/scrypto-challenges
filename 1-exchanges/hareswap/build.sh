#!/usr/bin/env bash
set -e
scrypto build && (cd hare && cargo build)
