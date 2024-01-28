#!/usr/bin/env bash

function rust_lint() {
    cargo test
    cargo fmt
    cargo clippy --all

}

function git_lint() {
    pre-commit run --all-files
}

function main() {

    local cmd="$1"
    local extra_args="${*:2}"

    case $cmd in
    "rust")
        rust_lint "${extra_args}"
        ;;
    "git")
        git_lint "${extra_args}"
        ;;
    "all")
        rust_lint "${extra_args}"
        git_lint "${extra_args}"
        ;;
    esac
}

main "$@"
