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
pub struct ChangeUserPassword {
    pub id: i32,
    pub oldpassword: String,
    pub newpassword: String,
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

impl<T> GenericResponse<T> {
    pub fn default_error(msg: &str) -> Self {
        Self {
            code: 1,
            status: String::from(msg),
            data: None,
        }
    }

    pub fn no_data() -> Self {
        Self {
            code: 1,
            status: String::from("No data found"),
            data: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Member {
    pub id: i32,
    pub surname: String,
    pub firstname: String,
    pub othernames: Option<String>,
    pub dob: Option<chrono::NaiveDate>,
    pub gender: Option<String>,
    pub maritalstatus: Option<String>,
    pub employed: Option<String>,
    pub occupation: Option<String>,
    pub company: Option<String>,
    pub companylocation: Option<String>,
    pub residence: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub passport: Option<String>,
    pub datecreated: Option<chrono::NaiveDateTime>,
    pub status: Option<String>,
    pub modified_date: Option<chrono::NaiveDateTime>,
    pub presbytery: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InputMember {
    pub id: Option<i32>,
    pub surname: String,
    pub firstname: String,
    pub othernames: String,
    pub dob: chrono::NaiveDate,
    pub gender: String,
    pub maritalstatus: String,
    pub employed: String,
    pub occupation: String,
    pub company: String,
    pub companylocation: String,
    pub residence: String,
    pub mobile: String,
    pub email: String,
    pub presbytery: String,
}

#[derive(Debug, Insertable)]
#[table_name = "members"]
pub struct NewMember<'a> {
    pub surname: &'a str,
    pub firstname: &'a str,
    pub othernames: &'a str,
    pub dob: chrono::NaiveDate,
    pub gender: &'a str,
    pub maritalstatus: &'a str,
    pub employed: &'a str,
    pub occupation: &'a str,
    pub company: &'a str,
    pub companylocation: &'a str,
    pub residence: &'a str,
    pub mobile: &'a str,
    pub email: &'a str,
    pub presbytery: &'a str,
    pub datecreated: chrono::NaiveDateTime,
    pub status: &'a str,
}