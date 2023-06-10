#!/usr/bin/env bash

ROOT="$HOME"

function setup() {
    if ! [ -x "${ROOT}/.cargo/bin/diesel" ]; then
        cargo install diesel_cli --force --no-default-features --features postgres
    fi

    "${ROOT}/.cargo/bin/diesel" database reset
}

function main() {
    local cmd="$1"
    local extra_args="${*:2}"

    case $cmd in
    "setup")
        setup "${extra_args}"
        ;;
    esac
}

main "$@"
