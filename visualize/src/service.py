from model import KlineEntry
from pandas import DataFrame
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
            LIMIT 200
            """
        )
        entrys.reverse()
        return map(lambda e: KlineEntry(e), entrys)


class Converter:
    def klines_to_dataframe(entrys: list[KlineEntry]) -> DataFrame:
        for i in entrys:
            print(i)
        data = map(lambda e: e.to_data(), entrys)
        price = []
        time = []
        for p, t in data:
            price.append(p)
            time.append(t)
        df = DataFrame(data=data, columns=["open", "close", "low", "high", "volume"])
        # print(df)

        return df
