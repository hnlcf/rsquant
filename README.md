# Quant Trader

## Build in Host

### Requirements

- **System**: Ubuntu 22.04
- **Language**: Rust 1.65, Python 3.10
- **Dependencies**: pkg-config, libssl-dev, libsqlite3-dev, libpq, postgresql

### Commands

```bash
./quant_trader.sh setup
```

## Build in Docker

### Requirements

- **Images**: Ubuntu 22.04

### Commands

```bash
./quant_trader.sh docker-build
./quant_trader.sh docker-into

./quant_trader.sh setup
```

## Run

### Launch data server

```bash
./quant_trader.sh run
```

### Launch web server

```bash
./quant_trader.sh web
```
