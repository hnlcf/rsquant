// @generated automatically by Diesel CLI.

diesel::table! {
    assets_kline_data (id) {
        id -> Int4,
        symbol -> Varchar,
        interval -> Varchar,
        open_time -> Timestamptz,
        open_price -> Varchar,
        high_price -> Varchar,
        low_price -> Varchar,
        close_price -> Varchar,
        volume -> Varchar,
        close_time -> Timestamptz,
        quote_asset_volume -> Varchar,
        trades_num -> Int8,
        buy_base_asset_volume -> Varchar,
        buy_quote_asset_volume -> Varchar,
        ignore_field -> Varchar,
    }
}

diesel::table! {
    assets_ticker_price_data (id) {
        id -> Int4,
        symbol -> Varchar,
        price -> Varchar,
        update_time -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(assets_kline_data, assets_ticker_price_data,);
