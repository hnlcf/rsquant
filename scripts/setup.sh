#!/usr/bin/env bash

ROOT=$(pwd)
BIN_NAME="quant_trader"
QUANT_ENVS=""

function setup_envs() {
  local ENV_VARS=$(grep -v '^#' .env | xargs -d '\n')

  QUANT_ENVS="${QUANT_ENVS} ${ENV_VARS}"
}

function build_release ()
{
  cargo build --release
}

function build_debug ()
{
  cargo build 
}

function run ()
{
  build_release
  env $QUANT_ENVS "${ROOT}/target/release/${BIN_NAME}" > "${ROOT}/quant_dev.log" 2>&1 &
}

function test ()
{
  build_debug 
  cargo nextest run $@
}

function main(){
  setup_envs

  local cmd="$1"
  local extra_args="${@:2}"

  case $cmd in
      "run")
         run
        ;;
      "build")
         build_release
        ;;
      "test")
         test "${extra_args}"
        ;;
   esac
}

main "$@"
