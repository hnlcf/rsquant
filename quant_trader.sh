#!/usr/bin/env bash

ROOT=$(pwd)
QUANT_ENVS=""

function setup_envs() {
  local ENV_VARS=$(grep -v '^#' .env | xargs -d '\n')

  QUANT_ENVS="${QUANT_ENVS} ${ENV_VARS}"
}

function main() {
    setup_envs

    local cmd="$1"
    local extra_args="${*:2}"
    local setup_sh="${ROOT}/scripts/setup.sh"
    local check_sh="${ROOT}/scripts/check.sh"

    case $cmd in
        "run")
          env "${QUANT_ENVS}" bash "${setup_sh}" run
          ;;
        "build")
          env "${QUANT_ENVS}" bash "${setup_sh}" build
          ;;
        "test")
          env "${QUANT_ENVS}" bash "${setup_sh}" test ${extra_args}
          ;;
        "lint")
          env "${QUANT_ENVS}" bash "${check_sh}"
          ;;
    esac
}

main "$@"
