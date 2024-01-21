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
    local docker_sh="${ROOT}/scripts/docker.sh"

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
        bash "${rust_sh}" run "${extra_args}"
        ;;
    "build")
        bash "${rust_sh}" build "${extra_args}"
        ;;
    "test")
        bash "${rust_sh}" test "${extra_args}"
        ;;
    "setup-docker")
        bash "${docker_sh}" build
        bash "${docker_sh}" into
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
