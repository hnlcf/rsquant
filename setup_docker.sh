#!/usr/bin/env bash

## Create database container and map port to 5433
docker container create -it --name quant-db -e POSTGRES_PASSWORD=postgres -p 5433:5432 postgres:latest

## Create build docker
docker container create -it --name quant-dev -v "$(pwd)":/app/quant_trader:z --network=host ubuntu:latest
docker exec -it quant-dev /bin/bash

## Install dependency
apt-get update -y && apt-get upgrade -y
apt-get install -y \
    apt-utils \
    tzdata \
    build-essential \
    vim \
    cargo \
    pkg-config \
    libssl-dev \
    libsqlite3-dev

## Switch rust crate mirror
mkdir -p ~/.cargo
echo " \
[source.crates-io] \
replace-with = 'mirror' \
[source.mirror] \
registry = \"https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git\" \
" >>~/.cargo/config

## Build project
./quant_trader build

#!!! Run in host for set use privilige
sudo chown -R "$USER" target/
