// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        display_name -> Text,
        created_at -> Timestamptz,
    }
}
