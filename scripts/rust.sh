#!/usr/bin/env bash

ROOT=$(pwd)
BIN_NAME="quant_trader"

function build() {
    cargo build
    cargo build --release
}

function run() {
    if [ "$RUST_BUILD_DEBUG" = 'ON' ]; then
        "${ROOT}/target/debug/${BIN_NAME}" >/dev/null 2>&1 &
    else
        "${ROOT}/target/release/${BIN_NAME}" >/dev/null 2>&1 &
    fi
}

function test() {
    build
    cargo nextest run "$@"
}

function setup() {
    mkdir -p log
    build
}

function main() {
    local cmd="$1"
    local extra_args="${*:2}"

    case $cmd in
    "setup")
        setup
        ;;
    "run")
        run
        ;;
    "build")
        build
        ;;
    "test")
        test "${extra_args}"
        ;;
    esac
}

main "$@"
