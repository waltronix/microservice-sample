// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Uuid,
        isbn -> Text,
        title -> Text,
        author -> Text,
        created_at -> Timestamptz,
    }
}
