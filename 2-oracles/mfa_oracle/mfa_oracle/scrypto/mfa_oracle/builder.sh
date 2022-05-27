#!/usr/bin/env bash
set -e
SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]:-$0}"; )" &> /dev/null && pwd 2> /dev/null; )";
HERE=$SCRIPT_DIR

if [ -z "$TESTING" ]; then
    cargo build --release --target wasm32-unknown-unknown
else
    cargo build --features testing --release --target wasm32-unknown-unknown
fi

wasm2wat=wasm2wat
wasmopt=wasm-opt
wasmsnip=$HERE/../../../..//wasm-snip/target/debug/wasm-snip
TARGET=$HERE/target/wasm32-unknown-unknown/release
TMP=$HERE/target/tmp
SRC=$TARGET/mfa_oracle.wasm
OSRC=$SRC

$wasm2wat $SRC > $SRC.wat
$wasmsnip $SRC -o $SRC.snip.wasm unfindablefunctionname
$wasm2wat $SRC.snip.wasm > $SRC.snip.wat
$wasmopt --dce -Oz -o $SRC.snip.opt.wasm $SRC.snip.wasm
$wasm2wat $SRC.snip.opt.wasm > $SRC.snip.opt.wat

# send pass (no longer needed?)
SRC=$SRC.snip.opt.wasm
$wasmsnip $SRC -o $SRC.snip.wasm unfindablefunctionname
$wasm2wat $SRC.snip.wasm > $SRC.snip.wat
$wasmopt --dce -Oz -o $SRC.snip.opt.wasm $SRC.snip.wasm
$wasm2wat $SRC.snip.opt.wasm > $SRC.snip.opt.wat

# copy back over original
cp $SRC.snip.opt.wasm $OSRC

# dist or testing
if [ -z "$TESTING" ]; then
    cp $OSRC $HERE/../../public/
else
    cp $OSRC $TARGET/testing.wasm
fi
