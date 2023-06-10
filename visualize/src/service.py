from model import KlineEntry
from postgresql import setup_postgres


class Service:
    def __init__(self):
        self.conn = setup_postgres()

    def get_kline(self, symbol: str, interval: str) -> list[KlineEntry]:
        entrys = self.conn.execute(
            f"""
            SELECT id, symbol, interval, open_price, high_price, low_price, close_price, volume, open_time, close_time
            FROM assets_kline_data
            WHERE symbol = '{symbol}' and interval = '{interval}'
            ORDER BY open_time DESC
            LIMIT 500
            """
        )
        entrys.reverse()
        return map(lambda e: KlineEntry(e), entrys)
