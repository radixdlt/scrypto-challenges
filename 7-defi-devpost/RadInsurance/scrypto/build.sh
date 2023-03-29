#!/bin/bash

set -x
set -e

cd "$(dirname "$0")"

(cd rad_insurance; scrypto build; cp target/wasm32-unknown-unknown/release/rad_insurance.wasm ../../public;)