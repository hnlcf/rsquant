-- Your SQL goes here

CREATE TABLE assets_ticker_price_data (
  id SERIAL PRIMARY KEY,
  symbol VARCHAR NOT NULL,
  price VARCHAR NOT NULL,
  update_time TIMESTAMPTZ NOT NULL
)
