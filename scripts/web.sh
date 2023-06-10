#!/usr/bin/env bash

ROOT=$(pwd)
PY_DIR="${ROOT}/visualize"
PY_SRC_DIR="${PY_DIR}/src"

function setup() {
    pip install -i https://pypi.tuna.tsinghua.edu.cn/simple -r "${ROOT}/visualize/requirements.txt"
}

function run() {
    python3 "${PY_SRC_DIR}/app.py"
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
