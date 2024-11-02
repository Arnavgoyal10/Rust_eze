// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Uuid,
        account_holder_name -> Varchar,
        created_at -> Timestamp,
        status -> Varchar,
    }
}

diesel::table! {
    records (transaction_id) {
        transaction_id -> Uuid,
        account_id_from -> Nullable<Uuid>,
        account_id_to -> Nullable<Uuid>,
        account_holder_from -> Varchar,
        account_holder_to -> Varchar,
        amount -> Float8,
        currency -> Varchar,
        created_at -> Timestamp,
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
    transactions (transaction_id) {
        transaction_id -> Uuid,
        sub_account_id_from -> Nullable<Uuid>,
        sub_account_id_to -> Nullable<Uuid>,
        amount -> Float8,
        transfer_currency -> Varchar,
        transaction_date -> Timestamp,
    }
}

diesel::joinable!(sub_accounts -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    records,
    sub_accounts,
    transactions,
);
