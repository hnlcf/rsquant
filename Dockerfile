FROM ubuntu:jammy
LABEL authors="changfeng"

ARG DEBIAN_FRONTEND=noninteractive
ARG CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse


## Replace software mirror
RUN sed -i "s#http://archive.ubuntu.com/ubuntu#http://mirrors.tuna.tsinghua.edu.cn/ubuntu#" /etc/apt/sources.list

## Update source
RUN apt-get update -y
RUN apt-get upgrade -y

## Install necessary dependencies
RUN apt-get install -y \
    tzdata \
    curl \
    pkg-config \
    libpq-dev \
    libssl-dev \
    libsqlite3-dev \
    python3 \
    python3-pip \
    postgresql

## Set timezone
RUN ln -fs /usr/share/zoneinfo/Asia/Shanghai /etc/localtime \
    && dpkg-reconfigure --frontend noninteractive tzdata

ENV TZ="Asia/Shanghai"

COPY . /app
WORKDIR /app

RUN chown -R 1000:1000 /app

# Create a new user with 1000:1000
RUN useradd -m -u 1000 -s /bin/bash -d /home/noroot noroot
USER noroot

## Setup rust config
RUN mkdir -p /home/noroot/.cargo

RUN export RUSTUP_DIST_SERVER=https://rsproxy.cn && \
    export RUSTUP_UPDATE_ROOT=https://rsproxy.cn/rustup && \
    curl -o /tmp/rustup-init https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup/dist/x86_64-unknown-linux-musl/rustup-init && \
    chmod +x /tmp/rustup-init && \
    /tmp/rustup-init -y --profile minimal --default-toolchain stable-x86_64-unknown-linux-gnu --default-host x86_64-unknown-linux-gnu --no-modify-path && \
    rm -f /tmp/rustup-init && \
    /home/noroot/.cargo/bin/rustup default nightly && \
    /home/noroot/.cargo/bin/rustup target add x86_64-unknown-linux-musl && \
    echo '. $HOME/.cargo/env' >> /home/noroot/.bashrc

RUN echo " \n\
    [source.crates-io] \n\
    replace-with = 'mirror' \n\
    [source.mirror] \n\
    registry = 'https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git' \n\
    "> /home/noroot/.cargo/config

## Setup python config
RUN mkdir -p /home/noroot/.config/pip

RUN echo " \n\
    [global] \n\
    index-url = https://pypi.tuna.tsinghua.edu.cn/simple \n\
    break-system-packages = true \n\
    "> /home/noroot/.config/pip/pip.conf
