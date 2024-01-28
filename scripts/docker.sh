#!/usr/bin/env bash

function build_image() {
    docker build --network=host -t quant-dev:latest .
}

function into_container() {
    docker run -it --name rsquant --network=host quant-dev:latest
}

function main() {
    local cmd="$1"
    local extra_args="${*:2}"

    case $cmd in
    "build")
        build_image "${extra_args}"
        ;;
    "into")
        into_container "${extra_args}"
        ;;
    esac
}

main "$@"
