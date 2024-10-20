use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::accounts;
use crate::schema::sub_accounts;
// use crate::schema::sub_accounts::dsl::sub_accounts;
// use crate::schema::sub_accounts::dsl::*;




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
    pub account_id: Option<Uuid>, // Nullable, hence Option<Uuid>
    pub currency: &'a str,
    pub balance: f64,
}