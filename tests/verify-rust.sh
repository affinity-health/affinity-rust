#!/bin/sh
set -eu
cp -R /source /tmp/sdk
cd /tmp/sdk
mkdir -p tests
cp /tests/rust_smoke.rs tests/transport.rs
if [ -f /tests/approved-rust.rs ]; then cp /tests/approved-rust.rs tests/approved.rs; fi
cargo test
cargo package --allow-dirty --no-verify
