#!/bin/sh
set -eu
cp -R /source /tmp/sdk
cd /tmp/sdk
mkdir -p tests
cp /tests/rust_smoke.rs tests/transport.rs
cargo test
cargo package --allow-dirty --no-verify
