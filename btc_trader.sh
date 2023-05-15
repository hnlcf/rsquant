#!/usr/bin/env bash

ROOT=$(pwd)

function main() {
    local cmd="$1"
    local setup_sh="${ROOT}/scripts/setup.sh"
    local check_sh="${ROOT}/scripts/check.sh"

    case $cmd in
        "test")
          bash "${setup_sh}"
          ;;
        "lint")
          bash "${check_sh}"
          ;;
    esac
}

main "$@"
