FROM ubuntu:latest
LABEL authors="changfeng"

ARG DEBIAN_FRONTEND=noninteractive
ARG CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

COPY . /app
WORKDIR /app

RUN sed -i "s#http://archive.ubuntu.com/ubuntu#http://mirrors.tuna.tsinghua.edu.cn/ubuntu#" /etc/apt/sources.list

RUN apt-get update -y
RUN apt-get upgrade -y

RUN apt-get install -y \
    tzdata \
    pkg-config \
    libpq-dev \
    libssl-dev \
    libsqlite3-dev \
    cargo \
    rust-gdb \
    python3 \
    python3-pip \
    postgresql
RUN ln -fs /usr/share/zoneinfo/Asia/Shanghai /etc/localtime \
 && dpkg-reconfigure --frontend noninteractive tzdata

ENV TZ="Asia/Shanghai"

RUN mkdir -p /root/.cargo

RUN echo " \n\
[source.crates-io] \n\
replace-with = 'mirror' \n\
[source.mirror] \n\
registry = 'https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git' \n\
"> /root/.cargo/config
