#!/bin/bash

set -x
set -e

cd "$(dirname "$0")"

(cd epoch_duration_oracle; scrypto build)