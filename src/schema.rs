// @generated automatically by Diesel CLI.

diesel::table! {
    memories (id) {
        id -> Nullable<Integer>,
        image -> Text,
        description -> Text,
        date -> Text,
    }
}
