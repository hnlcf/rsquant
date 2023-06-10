#!/usr/bin/env bash

ROOT=$(pwd)

function setup_envs() {
    export "$(grep -v '^#' .env | xargs -d '\n')"
}

function main() {
    setup_envs

    local cmd="$1"
    local extra_args="${*:2}"
    local db_sh="${ROOT}/scripts/database.sh"
    local web_sh="${ROOT}/scripts/web.sh"
    local rust_sh="${ROOT}/scripts/rust.sh"
    local check_sh="${ROOT}/scripts/check.sh"

    case $cmd in
    "setup")
        bash "${db_sh}" setup
        bash "${web_sh}" setup
        bash "${rust_sh}" setup
        ;;
    "web")
        bash "${web_sh}" run
        ;;
    "run")
        bash "${rust_sh}" run
        ;;
    "build")
        bash "${rust_sh}" build
        ;;
    "test")
        bash "${rust_sh}" test "${extra_args}"
        ;;
    "lint-rs")
        bash "${check_sh}" rust
        ;;
    "lint-git")
        bash "${check_sh}" git
        ;;
    esac
}

main "$@"
