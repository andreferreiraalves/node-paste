// @generated automatically by Diesel CLI.

diesel::table! {
    records (id) {
        id -> Uuid,
        content -> Nullable<Varchar>,
        file_name -> Nullable<Varchar>,
    }
}
