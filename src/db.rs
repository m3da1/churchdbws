use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::ExpressionMethods;
use crate::{
    diesel::QueryDsl,
    model::{GenericResponse, LoginUser, NewUser, User},
    util,
};
use crate::{diesel::RunQueryDsl, model::InputUser};
use actix_web::web;
use diesel::insert_into;
// use diesel::dsl::{delete, insert_into};

pub fn get_all_users(
    pool: web::Data<Pool>,
) -> Result<GenericResponse<Vec<User>>, diesel::result::Error> {
    let mut resp: GenericResponse<Vec<User>> = GenericResponse {
        code: 1,
        status: String::from("No data found"),
        data: None,
    };
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    if items.len() > 0 {
        resp.code = 0;
        resp.status = String::from("Success");
        resp.data = Some(items);
    }
    Ok(resp)
}

pub fn get_user_by_userid(
    pool: web::Data<Pool>,
    user_id: i32,
) -> Result<GenericResponse<User>, diesel::result::Error> {
    let mut resp: GenericResponse<User> = GenericResponse {
        code: 1,
        status: String::from("No data found"),
        data: None,
    };
    let conn = pool.get().unwrap();
    let data = users.find(user_id).first::<User>(&conn);
    if let Ok(u) = data {
        resp.code = 0;
        resp.status = String::from("Success");
        resp.data = Some(u);
    }
    Ok(resp)
}

pub fn add_single_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<GenericResponse<String>, diesel::result::Error> {
    let mut resp: GenericResponse<String> = GenericResponse {
        code: 1,
        status: String::from("User creation failed"),
        data: None,
    };
    let conn = db.get().unwrap();
    let passwd = util::encryptpass(&item.password);
    let new_user = NewUser {
        username: &item.username,
        password: passwd.as_str(),
        created_by: 1,
        datecreated: chrono::Local::now().naive_local(),
        msisdn: &item.mobile,
        status: "ACTIVE",
        firstname: &item.firstname,
        lastname: &item.lastname,
    };
    let result = insert_into(users).values(&new_user).execute(&conn)?;
    if result > 0 {
        resp.code = 0;
        resp.status = String::from("Success");
    }
    Ok(resp)
}

pub fn perform_login_user(
    db: web::Data<Pool>,
    item: web::Json<LoginUser>,
) -> Result<GenericResponse<User>, diesel::result::Error> {
    let mut resp: GenericResponse<User> = GenericResponse {
        code: 1,
        status: String::from("Invalid username or password"),
        data: None,
    };
    let conn = db.get().unwrap();
    let encoded_passwd = util::encryptpass(&item.password);
    let result = users
        .filter(username.eq(&item.username))
        .filter(password.eq(encoded_passwd))
        .first::<User>(&conn);
    match result {
        Ok(user) => {
            resp.code = 0;
            resp.status = String::from("Success");
            resp.data = Some(user);
        }
        Err(e) => {
            println!("Login error: {:?}", e);
        }
    }
    Ok(resp)
}

pub fn update_single_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<GenericResponse<String>, diesel::result::Error> {
    let mut resp: GenericResponse<String> = GenericResponse {
        code: 1,
        status: String::from("User update failed"),
        data: None,
    };
    let conn = db.get().unwrap();
    let userinfo: Result<User, diesel::result::Error> =
        users.find(&item.id.unwrap()).first::<User>(&conn);
    match userinfo {
        Ok(mut u) => {
            u.firstname = Some(item.firstname.clone());
            u.lastname = Some(item.lastname.clone());
            u.msisdn = Some(item.mobile.clone());
            u.username = Some(item.username.clone());
            u.lastupdatetime = Some(chrono::Local::now().naive_local());
            let updated_result = diesel::update(users.find(&item.id.unwrap())).set(&u).execute(&conn).unwrap();
            println!("Updated Result: {:?}", updated_result);
            resp.code = 0;
            resp.status = String::from("Success");
        }
        Err(e) => {
            println!("User not found: ErrorMessage: {:?}", e);
            resp.status = String::from("User not found");
        }
    }
    Ok(resp)
}
