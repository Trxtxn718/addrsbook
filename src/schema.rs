// @generated automatically by Diesel CLI.

diesel::table! {
    vlans (id) {
        id -> Int4,
        vid -> Int4,
        name -> Text,
    }
}
