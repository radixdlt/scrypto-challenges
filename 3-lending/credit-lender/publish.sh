#!/bin/bash

set -x
set -e

cd "$(dirname "$0")"

(cd pte-manifest-compiler; wasm-pack build --target web; wasm-pack publish --target web)

(cd pte-browser-extension-sdk; npm install; npm run build; npm publish)

(cd pte-sdk; npm install; npm run build; npm publish)