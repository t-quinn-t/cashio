// @generated automatically by Diesel CLI.

diesel::table! {
    records (id) {
        id -> Integer,
        name -> Text,
        cents -> Integer,
        date -> Nullable<Text>,
        category -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}
