// @generated automatically by Diesel CLI.

diesel::table! {
    memories (id) {
        id -> Integer,
        image -> Text,
        description -> Text,
        date -> Text,
    }
}
