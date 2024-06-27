// @generated automatically by Diesel CLI.

diesel::table! {
    records (id) {
        id -> Uuid,
        message -> Nullable<Varchar>,
        file_name -> Nullable<Varchar>,
    }
}
