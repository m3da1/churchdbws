use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub created_by: Option<i32>,
    pub datecreated: Option<chrono::NaiveDateTime>,
    pub msisdn: Option<String>,
    pub status: Option<String>,
    pub lastlogindate: Option<chrono::NaiveDateTime>,
    pub lastupdatetime: Option<chrono::NaiveDateTime>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InputUser {
    pub id: Option<i32>,
    pub firstname: String,
    pub lastname: String,
    pub mobile: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub created_by: i32,
    pub datecreated: chrono::NaiveDateTime,
    pub msisdn: &'a str,
    pub status: &'a str,
    pub firstname: &'a str,
    pub lastname: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericResponse<T> {
    pub code: u8,
    pub status: String,
    pub data: Option<T>,
}