# Quant Trader

## Build

### Requirements of Host

- **System**: Ubuntu 22.04
- **Language**: Rust 1.65
- **Dependencies**: pkg-config, libssl-dev, libpq, postgresql, poetry

### Requirements of Docker

- **Images**: Ubuntu 22.04

### Commands

```bash
./quant_trader.sh setup-docker

./quant_trader.sh setup

./quant_trader.sh build
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
