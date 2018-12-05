#!/usr/bin/env bash
RUST_BACKTRACE=1 cargo run --verbose --bin day_$1 < fixtures/day_$1.in