<div align="center">
    <!-- <img class="fit-picture" src="docs/assets/quant-trader-avatar.png" width=120px alt="Project Avatar" > -->
    <h1> rsquant </h1>
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
- **Dependencies**: pkg-config, libssl-dev, libpq, postgresql

### Commands

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

## Build in Docker

### Requirements

- **Images**: `clux/muslrust:nightly`

### Commands

```bash
docker run -v $PWD:/volume --rm -t clux/muslrust:nightly bash -c 'cargo build --release --target x86_64-unknown-linux-musl'
```

## Configuration

Before using it, you need to configure it correctly in the config file.

### Examples

```json
{
  "basic": {
    "symbol": "BTCUSDT",
    "interval": "Minutes30", // kline interval
    "total_currency": 100, // USDT
    "duration": 300 // seconds, polling interval of kline data
  },
  "api_credentials": {
    "type": "binance",
    "signature_type": "HMAC",
    "api_key": "<API-KEY>",
    "api_secret": "<API-SECRET>"
  },
  "email": {
    "from_email": "XXXX <XXX@gmail.com>",
    "to_emails": ["YYYY <YYY@gmail.com>"],
    "from_passwd": "<SMTP_GMAIL_PASSWD>",
    "smtp_addr": "smtp.gmail.com"
  },
  "network": {
    "proxy": {
      "https_proxy": "xxxx"
    }
  },
  "log": {
    "log_path": "log/"
  },
  "database": {
    "type": "postgresql",
    "pg_addr": "postgres://postgres:postgres@localhost:5432/rsquant"
  },
  "market": {
    "data": {
      "type": "kline"
    }
  },
  "strategy": {
    "signal": {
      "type": "macd"
    }
  }
}
```

### Description

1. `basic`: Provide basic trade informations.
2. `api_credentials`: It is your credentials manually generated in Binance account, and currently only supports HMAC.
3. `email`: You can set the sender's email account(`from_email`), password(`from_passwd`), server address(`smtp_addr`) and subscriber's email list(`to_emails`).
4. `network.proxy` **(Optional)**: You can set the address of the network proxy.
5. `log`: You need to set the path of log file **(The path must be exists)**.
6. `database.postgresql`: You need to set the address of postgresql database.

## Run

```bash
docker-compose up -d

QUANT_LOG_LEVEL=debug ./target/x86_64-unknown-linux-musl/release/rsquant --config ./rsquant.json
```

## License

[MIT](./LICENSE)
