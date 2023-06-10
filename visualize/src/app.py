from draw import Drawer
from flask import Flask
from markupsafe import Markup
from model import KlineEntry
from postgresql import setup_postgres
from service import Service

APP = Flask(__name__)
SERVICE = Service()


def launch_server():
    print("Launch server in Debug mode.")
    APP.run(debug=True, host="0.0.0.0", port="5000")


@APP.route("/")
def hello_world():
    return "<h1>This is quant trader!</1>"


@APP.route("/kline/<symbol>/<interval>")
def show_kline(symbol, interval):
    """
    symbol: 'BTCUSDT', 'ETHUSDT' ...
    interval: '1m', '5m', '30m', '1h', '4h', '1d'
    """
    klines = SERVICE.get_kline(symbol=symbol, interval=interval)
    grid = Drawer.draw_kline(klines=klines, symbol=symbol, interval=interval)
    return Markup(grid.render_embed())


if __name__ == "__main__":
    launch_server()
