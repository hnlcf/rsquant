from datetime import time


class KlineEntry:
    def __init__(self, entry: tuple[any, ...]) -> None:
        (
            entry_id,
            symbol,
            interval,
            open_price,
            high_price,
            low_price,
            close_price,
            volume,
            open_time,
            close_time,
        ) = entry
        self.symbol = symbol
        self.interval = interval
        self.open_time = open_time.astimezone().strftime("%Y-%m-%d %H:%M:%S")
        self.close_time = close_time.astimezone().strftime("%Y-%m-%d %H:%M:%S")
        self.open_price = float(open_price)
        self.high_price = float(high_price)
        self.low_price = float(low_price)
        self.close_price = float(close_price)
        self.volume = float(volume)

    def __str__(self) -> str:
        return f"""Kline(
        symbol: {self.symbol},
        interval: {self.interval},
        open_time: {self.open_time},
        close_time: {self.close_time},
        open_price: {self.open_price},
        high_price: {self.high_price},
        low_price: {self.low_price},
        close_price: {self.close_price},
        volume: {self.volume}
        )"""
