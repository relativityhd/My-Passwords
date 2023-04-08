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
        user_id -> Unsigned<Integer>,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Integer>,
        username -> Varchar,
        pass -> Varchar,
    }
}

diesel::joinable!(passwords -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    passwords,
    users,
);
