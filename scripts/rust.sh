#!/usr/bin/env bash

ROOT=$(pwd)
BIN_NAME="quant_trader"

function build() {
    if [ "$1" = "-d" ] || [ "$1" = "--debug" ]; then
        cargo build
    else
        cargo build --release
    fi
}

function run() {
    if [ "$1" = "-d" ] || [ "$1" = "--debug" ]; then
        build "--debug"
        "${ROOT}/target/debug/${BIN_NAME}" >/dev/null 2>&1 &
    else
        build "--release"
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
        run "${extra_args}"
        ;;
    "build")
        build "${extra_args}"
        ;;
    "test")
        test "${extra_args}"
        ;;
    esac
}

main "$@"
