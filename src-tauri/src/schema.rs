// @generated automatically by Diesel CLI.

diesel::table! {
    passwords (id) {
        id -> Unsigned<Integer>,
        account_name -> Varchar,
        secret -> Varchar,
        institution -> Varchar,
        industry -> Varchar,
        is_legacy -> Bool,
        is_work -> Bool,
        date_created -> Datetime,
    }
}
