#!/usr/bin/env bash

ROOT=$(pwd)
PY_DIR="${ROOT}/visualize"
PY_SRC_DIR="${PY_DIR}/src"

function setup() {
    if ! [ -x "${HOME}/.local/bin/poetry" ]; then
        curl -sSL https://install.python-poetry.org | python3 -
    fi

    "${HOME}/.local/bin/poetry" install
}

function run() {
    "${HOME}/.local/bin/poetry" run python3 "${PY_SRC_DIR}/app.py"
}

function main() {
    local cmd="$1"
    local extra_args="${*:2}"

    case $cmd in
    "setup")
        setup "${extra_args}"
        ;;
    "run")
        run "${extra_args}"
        ;;
    esac
}

main "$@"
