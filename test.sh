#!/usr/bin/env bash
RUST_BACKTRACE=1 cargo test --release --verbose --bin day_$1
