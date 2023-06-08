#!/usr/bin/env bash

ROOT=$(pwd)
PY_DIR="${ROOT}/visualize"
PY_SRC_DIR="${PY_DIR}/src"
PY_ENV_DIR="${PY_DIR}/.venv"

function setup() {
    source "${PY_ENV_DIR}/bin/activate"
}

function run() {
    python "${PY_SRC_DIR}/app.py"
}

function main() {
    setup

    local cmd="$1"
    local extra_args="${*:2}"

    case $cmd in
    "run")
        run "${extra_args}"
        ;;
    esac
}

main "$@"
