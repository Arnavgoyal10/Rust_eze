use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::accounts;
use crate::schema::sub_accounts;
use crate::schema::transactions;
use crate::schema::pending_transactions;
use crate::schema::scheduled_transactions;
use crate::schema::username_password;

pub const ADMIN_ACCOUNT_ID: Uuid = Uuid::from_u128(0);

#[derive(Queryable, Debug, QueryableByName, Selectable)]
#[diesel(table_name = accounts)]
pub struct Account {
   pub id: Uuid,
   pub account_holder_name: String,
   pub created_at: NaiveDateTime,
   pub status: String,
}

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount<'a> {
    pub account_holder_name: &'a str,
    pub status: &'a str,
}

#[derive(Queryable, Debug, QueryableByName, Selectable)]
#[diesel(table_name = sub_accounts)]
pub struct SubAccount {
    pub id: Uuid,
    pub account_id: Option<Uuid>,
    pub currency: String,
    pub balance: f64,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = sub_accounts)]
pub struct NewSubAccount<'a> {
    pub account_id: Option<Uuid>,
    pub currency: &'a str,
    pub balance: f64,
}

#[derive(Queryable, Debug, QueryableByName, Selectable)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub transaction_id: Uuid,
    pub sub_account_id_from: Option<Uuid>,
    pub sub_account_id_to: Option<Uuid>,
    pub amount: f64,
    pub transfer_currency: String,
    pub transaction_date: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction<'a> {
    pub sub_account_id_from: Option<Uuid>,
    pub sub_account_id_to: Option<Uuid>,
    pub amount: f64,
    pub transfer_currency: &'a str,
}

#[derive(Queryable, Debug, QueryableByName, Selectable)]
#[diesel(table_name = pending_transactions)]
pub struct PendingTransaction {
    pub id: Uuid,
    pub account_id_to_add: Option<Uuid>,
    pub amount: f64,
    pub transfer_currency: String,
    pub transaction_date: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = pending_transactions)]
pub struct NewPendingTransaction<'a> {
    pub account_id_to_add: Option<Uuid>,
    pub amount: f64,
    pub transfer_currency: &'a str,
}

#[derive(Queryable, Debug, QueryableByName, Selectable)]
#[diesel(table_name = username_password)]
pub struct UsernamePassword {
    pub username: String,
    pub passwd: String,
    pub totp_secret: Option<String>,
    pub account_id: Option<Uuid>,
}

#[derive(Insertable)]
#[diesel(table_name = username_password)]
pub struct NewUsernamePassword<'a> {
    pub username: &'a str,
    pub passwd: &'a str,
    pub totp_secret: Option<&'a str>,
    pub account_id: Option<Uuid>,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = scheduled_transactions)]
pub struct ScheduledTransaction {
    pub id: Uuid,
    pub from_account_id: Uuid,
    pub to_account_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub scheduled_date: NaiveDateTime,
    pub executed: bool,
}


#[derive(Insertable)]
#[diesel(table_name = scheduled_transactions)]
pub struct NewScheduledTransaction<'a> {
    pub from_account_id: Uuid,
    pub to_account_id: Uuid,
    pub amount: f64,
    pub currency: &'a str,
    pub scheduled_date: NaiveDateTime,
    pub executed: bool,
}