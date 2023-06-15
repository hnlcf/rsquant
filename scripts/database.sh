#!/usr/bin/env bash

function setup() {
    if ! [ -x "${HOME}/.cargo/bin/diesel" ]; then
        cargo install diesel_cli --force --no-default-features --features postgres
    fi

    "${HOME}/.cargo/bin/diesel" database reset
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
