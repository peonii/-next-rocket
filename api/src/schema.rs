// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        name -> Varchar,
        completed -> Bool,
    }
}
