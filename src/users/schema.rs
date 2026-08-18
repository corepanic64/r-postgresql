diesel::table! {
    users (id) {
        id -> Int4,
        email -> VarChar,
        username -> VarChar,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp
    }
}
