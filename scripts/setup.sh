#!/usr/bin/env bash

ROOT=$(pwd)

BINAN_API_ENV=""

function setup_envs() {
  local ENV_VARS=$(grep -v '^#' .env | xargs -d '\n')

  BINAN_API_ENV="${BINAN_API_ENV} ${ENV_VARS}"
}

function build_rs ()
{
  cargo build --all
}

function main ()
{
  setup_envs
  build_rs

  env $BINAN_API_ENV proxychains4 "${ROOT}/target/debug/btc_trader"
}

main "$@"
