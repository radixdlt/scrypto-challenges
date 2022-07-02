#!/bin/bash

set -x
set -e

(scrypto build; cp target/wasm32-unknown-unknown/release/lending_dapp.wasm public)

npm start