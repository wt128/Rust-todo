table! {
    todos (id) {
        id -> Unsigned<Bigint>,
        title -> Varchar,
        content -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
