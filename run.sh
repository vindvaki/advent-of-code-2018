#!/usr/bin/env bash
RUST_BACKTRACE=1 cargo run --release --verbose --bin day_$1 < fixtures/day_$1.in
