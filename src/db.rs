use super::schema::users::dsl::*;
use super::Pool;
use crate::{diesel::ExpressionMethods, model::ChangeUserPassword};
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
    let mut resp: GenericResponse<Vec<User>> = GenericResponse::no_data();
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
    let mut resp: GenericResponse<User> = GenericResponse::no_data();
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
    let mut resp = GenericResponse::default_error("User creation failed");
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
    let mut resp = GenericResponse::default_error("Invalid username or password");
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
    let mut resp = GenericResponse::default_error("User update failed");
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
            let updated_result = diesel::update(users.find(&item.id.unwrap()))
                .set(&u)
                .execute(&conn)
                .unwrap();
            println!("Updated Result: {:?}", updated_result);
            if updated_result > 0 {
                resp.code = 0;
                resp.status = String::from("Success");
            }
        }
        Err(e) => {
            println!("User not found: ErrorMessage: {:?}", e);
            resp.status = String::from("User not found");
        }
    }
    Ok(resp)
}

pub fn delete_single_user(
    db: web::Data<Pool>,
    user_id: i32,
) -> Result<GenericResponse<String>, diesel::result::Error> {
    let mut resp = GenericResponse::default_error("User deletion failed");
    let conn = db.get().unwrap();
    let userinfo: Result<User, diesel::result::Error> = users.find(user_id).first::<User>(&conn);
    if let Err(e) = userinfo {
        println!("Error message: {:?}", e);
        resp.status = String::from("User not found");
        return Ok(resp);
    }
    if let Ok(mut u) = userinfo {
        u.status = Some(String::from("DELETED"));
        u.lastupdatetime = Some(chrono::Local::now().naive_local());
        let updated = diesel::update(users.find(u.id))
            .set(&u)
            .execute(&conn)
            .unwrap();
        if updated > 0 {
            resp.code = 0;
            resp.status = String::from("Success");
        }
    }
    Ok(resp)
}

pub fn update_password(
    db: web::Data<Pool>,
    item: ChangeUserPassword,
) -> Result<GenericResponse<String>, diesel::result::Error> {
    let mut resp = GenericResponse::default_error("Password update failed");
    let conn = db.get().unwrap();
    if (&item.oldpassword).eq(&item.newpassword) {
        resp.status = String::from("Passwords are identical");
        return Ok(resp);
    }
    let userinfo: Result<User, diesel::result::Error> = users.find(&item.id).first::<User>(&conn);
    if let Err(_) = userinfo {
        resp.status = String::from("User not found");
        return Ok(resp);
    }
    let encoded_passwd = util::encryptpass(&item.oldpassword);
    if let Ok(mut user) = userinfo {
        if !encoded_passwd.eq(user.password.unwrap().as_str()) {
            resp.status = String::from("Invalid initial password");
            return Ok(resp);
        }
        user.password = Some(util::encryptpass(&item.newpassword));
        user.lastupdatetime = Some(chrono::Local::now().naive_local());
        let updated = diesel::update(users.find(user.id))
            .set(&user)
            .execute(&conn)
            .unwrap();
        if updated > 0 {
            resp.code = 0;
            resp.status = String::from("Success");
        }
    }
    Ok(resp)
}
