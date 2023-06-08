#!/usr/bin/env bash

ROOT=$(pwd)
BIN_NAME="quant_trader"

function build_release() {
    cargo build --release
}

function build_debug() {
    cargo build
}

function run() {
    if [ "$RUST_BUILD_DEBUG" = 'ON' ]; then
        "${ROOT}/target/debug/${BIN_NAME}" >/dev/null 2>&1 &
    else
        "${ROOT}/target/release/${BIN_NAME}" >/dev/null 2>&1 &
    fi
}

function test() {
    build_debug
    cargo nextest run "$@"
}

function main() {
    local cmd="$1"
    local extra_args="${*:2}"

    case $cmd in
    "run")
        run
        ;;
    "build")
        build_debug
        build_release
        ;;
    "test")
        test "${extra_args}"
        ;;
    esac
}

main "$@"
