from model import KlineEntry
from postgresql import PgConnection


class Service:
    def get_kline(conn: PgConnection, symbol: str) -> list[KlineEntry]:
        entrys = conn.execute(
            f"""
            SELECT id, symbol, open_price, high_price, low_price, close_price, volume, open_time, close_time
            FROM assets_kline_data
            WHERE symbol = '{symbol}'
            ORDER BY id DESC
            LIMIT 500
            """
        )
        entrys.reverse()
        return map(lambda e: KlineEntry(e), entrys)
