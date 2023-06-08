import os

import pandas
import psycopg2
from flask import Flask
from gevent.pywsgi import WSGIServer

app = Flask(__name__)


def launch_server():
    run_mode = os.getenv("FLASK_ENV")
    if run_mode == "development":
        print("Launch server in Debug mode.")
        app.run(debug=True, host="0.0.0.0", port="5000")
    else:
        print("Launch server in Prod mode.")
        http_server = WSGIServer(("", 5000), app)
        http_server.serve_forever()


@app.route("/")
def hello_world():
    return "<h1>Hello, World!</1>"


class PgOption:
    def __init__(self, db_name: str, db_user: str, db_passwd: str, db_host: str, db_port: str):
        self.db_name = db_name
        self.db_user = db_user
        self.db_passwd = db_passwd
        self.db_host = db_host
        self.db_port = db_port


class PgConnection:
    def __init__(self, option: PgOption):
        self.conn = psycopg2.connect(
            database=option.db_name,
            user=option.db_user,
            password=option.db_passwd,
            host=option.db_host,
            port=option.db_port,
        )
        self.cursor = self.conn.cursor()

    def __del__(self):
        self.cursor.close()
        self.conn.close()

    def execute(self, sql: str):
        self.cursor.execute(sql)
        result = self.cursor.fetchall()
        return result


class Kline:
    def __init__(self, entry: tuple[any, ...]) -> None:
        (
            entry_id,
            symbol,
            open_price,
            high_price,
            low_price,
            close_price,
            volume,
            open_time,
            close_time,
        ) = entry
        self.symbol = symbol
        self.open_time = (open_time.astimezone().strftime("%Y-%d-%m, %H:%M:%S"),)
        self.close_time = (close_time.astimezone().strftime("%Y-%d-%m, %H:%M:%S"),)
        self.open_price = float(open_price)
        self.high_price = float(high_price)
        self.low_price = float(low_price)
        self.close_price = float(close_price)
        self.volume = float(volume)

    def __str__(self) -> str:
        return f"""Kline(
        symbol: {self.symbol},
        open_time: {self.open_time},
        close_time: {self.close_time},
        open_price: {self.open_price},
        high_price: {self.high_price},
        low_price: {self.low_price},
        close_price: {self.close_price},
        volume: {self.volume}
        )"""


def get_kline(conn: PgConnection) -> list[Kline]:
    entrys = conn.execute(
        """
        SELECT id, symbol, open_price, high_price, low_price, close_price, volume, open_time, close_time
        FROM assets_kline_data
        ORDER BY id DESC
        LIMIT 500
        """
    )
    entrys.reverse()
    data = map(lambda e: Kline(e), entrys)

    return data


def setup_postgres() -> PgConnection:
    pg_option = PgOption(
        db_name="quant_db_test",
        db_user="postgres",
        db_passwd="postgres",
        db_host="localhost",
        db_port="5433",
    )
    pg_conn = PgConnection(pg_option)
    return pg_conn


if __name__ == "__main__":
    pg_conn = setup_postgres()
    data = get_kline(pg_conn)
    for i in data:
        print(i)
