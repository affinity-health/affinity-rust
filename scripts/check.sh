#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
docker run --rm -v "$PWD:/source:ro" -v "$PWD/tests:/tests:ro" rust:1.90 sh /tests/verify-rust.sh
