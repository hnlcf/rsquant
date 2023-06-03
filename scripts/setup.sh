#!/usr/bin/env bash

ROOT=$(pwd)
BIN_NAME="quant_trader"

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
  if [ "$BUILD_DEBUG" = 'ON' ]; then
    build_debug
    "${ROOT}/target/debug/${BIN_NAME}" >> "${ROOT}/log/quant_dev.log" 2>&1 &
  else
    build_release
    "${ROOT}/target/release/${BIN_NAME}" >> "${ROOT}/log/quant_dev.log" 2>&1 &
  fi
}

function test ()
{
  build_debug 
  cargo nextest run "$@"
}

function main(){
  local cmd="$1"
  local extra_args="${*:2}"

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
