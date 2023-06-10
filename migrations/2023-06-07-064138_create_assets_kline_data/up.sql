-- Your SQL goes here
CREATE TABLE assets_kline_data (
  id SERIAL PRIMARY KEY,
  symbol VARCHAR NOT NULL,
  interval VARCHAR NOT NULL,
  open_time TIMESTAMPTZ NOT NULL,
  open_price VARCHAR NOT NULL,
  high_price VARCHAR NOT NULL,
  low_price VARCHAR NOT NULL,
  close_price VARCHAR NOT NULL,
  volume VARCHAR NOT NULL,
  close_time TIMESTAMPTZ NOT NULL,
  quote_asset_volume VARCHAR NOT NULL,
  trades_num BIGINT NOT NULL,
  buy_base_asset_volume VARCHAR NOT NULL,
  buy_quote_asset_volume VARCHAR NOT NULL,
  ignore_field VARCHAR NOT NULL
)
