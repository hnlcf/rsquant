<div align="center">
    <img class="fit-picture" src="docs/assets/quant-trader-avatar.png" width=120px alt="Project Avatar" >
    <h1> Quant Trader </h1>
    <i> A quanting trade system of the cryptocurrency based on Rust. </i>
</div>

## Contents

- [Contents](#contents)
- [Introduction](#introduction)
- [Build in Host](#build-in-host)
  - [Requirements](#requirements)
  - [Commands](#commands)
- [Build in Docker](#build-in-docker)
  - [Requirements](#requirements-1)
  - [Commands](#commands-1)
- [Configuration](#configuration)
  - [Examples](#examples)
  - [Description](#description)
- [Run](#run)
  - [Launch data server](#launch-data-server)
  - [Launch web server](#launch-web-server)
- [License](#license)

## Introduction

Due to the direct use of [Binance Public API Connector Rust](https://github.com/binance/binance-spot-connector-rust), only [Binance](https://www.binance.com) is currently supported.

The following picture is the structure diagram of the project divided by modules.

![project-structure](docs/assets/project-structure.png)

The following is execution flow:

![exec-flow](docs/assets/exec-flow.png)

## Build in Host

### Requirements

- **System**: Ubuntu 22.04
- **Language**: Rust 1.75
- **Dependencies**: pkg-config, libssl-dev, libpq, postgresql, poetry

### Commands

```bash
./rsquant.sh setup

./rsquant.sh build
```

## Build in Docker

### Requirements

- **Images**: Ubuntu 22.04

### Commands

```bash
./rsquant.sh setup-docker

./rsquant.sh setup

./rsquant.sh build
```

## Configuration

Before using it, you need to configure it correctly in the config file.

### Examples

```toml
# File location: "$XDG_CONFIG_HOME/rsquant/config.toml"

[api_credentials.binance]
signature_type="HMAC"
api_key="<your-api-key>"
api_secret="<your-api-secret>"

[email]
from_email="Quant Trader <xxxx@gmail.com>"
to_emails=["hnlcf <xxxx@gmail.com>"]
from_passwd="xxxx"
smtp_addr="smtp.gmail.com"

[network.proxy]
# https_proxy="<protocol>://<proxy-host>:<port>"

[log]
log_path="log/output.log"

[database.postgresql]
pg_addr="postgres://postgres:postgres@localhost:5432/rsquant_db"

```

### Description

1. `api_credentials`: It is your credentials manually generated in Binance account, and currently only supports HMAC.
2. `email`: You can set the sender's email account(`from_email`), password(`from_passwd`), server address(`smtp_addr`) and subscriber's email list(`to_emails`).
3. `network.proxy` **(Optional)**: You can set the address of the network proxy.
4. `log`: You need to set the path of log file **(The path must be exists)**.
5. `database.postgresql`: You need to set the address of postgresql database.

## Run

### Launch data server

```bash
./rsquant.sh run
```

### Launch web server

```bash
./rsquant.sh web
```

## License

[MIT](./LICENSE)
