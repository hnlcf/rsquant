import sqlite3

import mplfinance as mpf
import pandas

if __name__ == "__main__":
    conn = sqlite3.connect("database/bitcoin.db")
    cursor = conn.cursor()

    cursor.execute(
        """
        SELECT id, name, open_price, high_price, low_price, close_price, volume, open_date_time
        FROM assets_kline_data
        ORDER BY id DESC
        LIMIT 500
        """
    )

    result = cursor.fetchall()
    result.reverse()

    data = []
    for i in result:
        (entry_id, name, open_price, high_price, low_price, close_price, volume, open_time) = i
        item = [
            open_time,
            float(open_price),
            float(high_price),
            float(low_price),
            float(close_price),
            float(volume),
        ]
        data.append(item)

    df = pandas.DataFrame(
        data, columns=["datetime", "open_price", "high_price", "low_price", "close_price", "volume"]
    )
    df["datetime"] = pandas.to_datetime(df["datetime"], format="%Y-%m-%d %H:%M:%S")
    df.set_index("datetime", inplace=True)

    mpf.plot(
        df,
        type="candle",
        title="ETHUSDT",
        columns=["open_price", "high_price", "low_price", "close_price", "volume"],
    )
