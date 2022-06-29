#!/bin/bash

set -x
set -e

(scrypto build; cp target/wasm32-unknown-unknown/release/loan_application.wasm public)