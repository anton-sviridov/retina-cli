// @generated automatically by Diesel CLI.

diesel::table! {
    detections (id) {
        id -> Integer,
        image -> Text,
        structure -> Text,
        date -> Text,
    }
}
