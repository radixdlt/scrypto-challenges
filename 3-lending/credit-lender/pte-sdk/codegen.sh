#!/bin/bash

set -x
set -e

cd "$(dirname "$0")"

rm -fr src/api

docker run --rm -v "${PWD}/..:/local" openapitools/openapi-generator-cli generate \
    -g typescript-fetch \
    --additional-properties=typescriptThreePlus=true \
    -i /local/pte-api-spec/api.yaml \
    -o /local/pte-sdk/src/api