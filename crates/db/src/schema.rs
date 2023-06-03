// @generated automatically by Diesel CLI.

diesel::table! {
    assets_ticker_price (id) {
        id -> Int4,
        name -> Text,
        price -> Text,
        unix_time -> Int4,
        date_time -> Text,
    }
}
