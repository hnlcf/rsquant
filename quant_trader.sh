#!/usr/bin/env bash

ROOT=$(pwd)

function main() {
    local cmd="$1"
    local extra_args="${@:2}"
    local setup_sh="${ROOT}/scripts/setup.sh"
    local check_sh="${ROOT}/scripts/check.sh"

    case $cmd in
        "run")
          bash "${setup_sh}" run
          ;;
        "build")
          bash "${setup_sh}" build
          ;;
        "test")
          bash "${setup_sh}" test ${extra_args}
          ;;
        "lint")
          bash "${check_sh}"
          ;;
    esac
}

main "$@"
