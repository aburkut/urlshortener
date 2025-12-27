// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Int4,
        url -> Text,
        short -> Text,
        created_at -> Timestamp,
        clicks -> Int4,
        expires_at -> Nullable<Timestamp>,
    }
}
