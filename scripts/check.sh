#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
./tests/with-fixtures.sh docker run --rm --network host -v "$PWD:/source:ro" -v "$PWD/tests:/tests:ro" rust:1.90 sh /tests/verify-rust.sh
