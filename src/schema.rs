// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Uuid,
        account_holder_name -> Varchar,
        created_at -> Timestamp,
        status -> Varchar,
        totp_secret -> Varchar,
    }
}

diesel::table! {
    sub_accounts (id) {
        id -> Uuid,
        account_id -> Nullable<Uuid>,
        currency -> Varchar,
        balance -> Float8,
        created_at -> Timestamp,
    }
}

diesel::table! {
    records (id) {
        transaction_id -> Uuid,
        account_id_from -> Nullable<Uuid>,
        account_id_to -> Nullable<Uuid>,
        created_at -> Timestamp,
        amount -> Float8,
        currency -> Varchar,
    }
}

diesel::joinable!(sub_accounts -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    sub_accounts,
);
