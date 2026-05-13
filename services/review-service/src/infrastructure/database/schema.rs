// @generated automatically by Diesel CLI.

diesel::table! {
    reviews (id) {
        id -> Uuid,
        book_id -> Uuid,
        user_id -> Uuid,
        rating -> Int2,
        body -> Text,
        created_at -> Timestamptz,
    }
}
