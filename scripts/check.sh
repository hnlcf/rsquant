#!/usr/bin/env bash

cargo test
cargo fmt -- --check
cargo clippy --all-targets -- -D clippy::all
