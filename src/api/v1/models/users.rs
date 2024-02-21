use diesel::prelude::*;
use chrono::NaiveDateTime;
use chrono::DateTime;
use chrono::offset::Utc;
use serde::Serialize;

#[derive(Debug)]
#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub is_admin: bool,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: String,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub password_hash: String,
    pub registered_at: DateTime<Utc>,
    pub last_login: Option<NaiveDateTime>,
    pub intro: Option<String>,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
}
