use super::schema::members::dsl::*;
use super::schema::stewardgroups::dsl::*;
use super::schema::users::dsl::*;
use super::Pool;
use crate::{
    diesel::ExpressionMethods,
    model::{
        ChangeUserPassword, InputMember, InputStewardgroup, Member, NewGroup, NewMember,
        Stewardgroup,
    },
};
use crate::{
    diesel::QueryDsl,
    model::{GenericResponse, LoginUser, NewUser, User},
    util,
};
use crate::{diesel::RunQueryDsl, model::InputUser};
use actix_web::web;

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
    let result = diesel::insert_into(users)
        .values(&new_user)
        .execute(&conn)?;
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

pub fn get_all_members(
    db: web::Data<Pool>,
) -> Result<GenericResponse<Vec<Member>>, diesel::result::Error> {
    let mut resp = GenericResponse::no_data();
    let conn = db.get().unwrap();
    let items = members.load::<Member>(&conn)?;
    if items.len() > 0 {
        resp.code = 0;
        resp.status = String::from("Success");
        resp.data = Some(items);
    }
    Ok(resp)
}

pub fn get_members_by_userid(
    db: web::Data<Pool>,
    user_id: i32,
) -> Result<GenericResponse<Member>, diesel::result::Error> {
    let mut resp = GenericResponse::no_data();
    let conn = db.get().unwrap();
    let data = members.find(user_id).first::<Member>(&conn);
    if let Ok(u) = data {
        resp.code = 0;
        resp.status = String::from("Success");
        resp.data = Some(u);
    }
    Ok(resp)
}

pub fn add_single_member(
    db: web::Data<Pool>,
    item: web::Json<InputMember>,
) -> Result<GenericResponse<String>, diesel::result::Error> {
    let mut resp = GenericResponse::default_error("Member creation failed");
    let conn = db.get().unwrap();
    let new_member = NewMember {
        surname: &item.surname,
        firstname: &item.firstname,
        othernames: &item.othernames,
        dob: item.dob,
        gender: &item.gender,
        maritalstatus: &item.maritalstatus,
        employed: &item.employed,
        occupation: &item.occupation,
        company: &item.company,
        companylocation: &item.companylocation,
        residence: &item.residence,
        mobile: &item.mobile,
        email: &item.email,
        presbytery: &item.presbytery,
        datecreated: chrono::Local::now().naive_local(),
        status: "ACTIVE",
    };
    let result = diesel::insert_into(members)
        .values(&new_member)
        .execute(&conn)?;
    if result > 0 {
        resp.code = 0;
        resp.status = String::from("Success");
    }
    Ok(resp)
}

pub fn update_single_member(
    db: web::Data<Pool>,
    item: web::Json<InputMember>,
) -> Result<GenericResponse<String>, diesel::result::Error> {
    let mut resp = GenericResponse::default_error("Member update failed");
    let conn = db.get().unwrap();
    let memberinfo: Result<Member, diesel::result::Error> =
        members.find(&item.id.unwrap()).first::<Member>(&conn);
    if let Err(_) = memberinfo {
        resp.status = String::from("Member not found");
        return Ok(resp);
    }
    if let Ok(mut member) = memberinfo {
        member.surname = item.surname.clone();
        member.firstname = item.firstname.clone();
        member.othernames = Some(item.othernames.clone());
        member.dob = Some(item.dob);
        member.gender = Some(item.gender.clone());
        member.maritalstatus = Some(item.maritalstatus.clone());
        member.employed = Some(item.employed.clone());
        member.occupation = Some(item.occupation.clone());
        member.company = Some(item.company.clone());
        member.companylocation = Some(item.companylocation.clone());
        member.residence = Some(item.residence.clone());
        member.mobile = Some(item.mobile.clone());
        member.email = Some(item.email.clone());
        member.presbytery = Some(item.presbytery.clone());
        member.modified_date = Some(chrono::Local::now().naive_local());
        let updated_result = diesel::update(members.find(&item.id.unwrap()))
            .set(&member)
            .execute(&conn)
            .unwrap();
        println!("Updated Result: {:?}", updated_result);
        if updated_result > 0 {
            resp.code = 0;
            resp.status = String::from("Success");
        }
    }
    Ok(resp)
}

pub fn delete_single_member(
    db: web::Data<Pool>,
    member_id: web::Path<i32>,
) -> Result<GenericResponse<String>, diesel::result::Error> {
    let mut resp = GenericResponse::default_error("Member deletion failed");
    let conn = db.get().unwrap();
    let memeberinfo: Result<Member, diesel::result::Error> =
        members.find(member_id.into_inner()).first::<Member>(&conn);
    match memeberinfo {
        Ok(mut member) => {
            member.status = Some(String::from("DELETED"));
            member.modified_date = Some(chrono::Local::now().naive_local());
            let updated = diesel::update(members.find(member.id))
                .set(&member)
                .execute(&conn)
                .unwrap();
            if updated > 0 {
                resp.code = 0;
                resp.status = String::from("Success");
            }
        }
        Err(e) => {
            println!("Member not found: ErrorMessage: {:?}", e);
            resp.status = String::from("Member not found");
        }
    }
    Ok(resp)
}

pub fn get_stewardship_groups(
    db: web::Data<Pool>,
) -> Result<GenericResponse<Vec<Stewardgroup>>, diesel::result::Error> {
    let mut resp = GenericResponse::no_data();
    let conn = db.get().unwrap();
    let items = stewardgroups.load::<Stewardgroup>(&conn)?;
    if items.len() > 0 {
        resp.code = 0;
        resp.status = String::from("Success");
        resp.data = Some(items);
    }
    Ok(resp)
}

pub fn get_single_steward_group(
    db: web::Data<Pool>,
    group_id: i32,
) -> Result<GenericResponse<Stewardgroup>, diesel::result::Error> {
    let mut resp = GenericResponse::no_data();
    let conn = db.get().unwrap();
    let group = stewardgroups.find(group_id).first::<Stewardgroup>(&conn);
    if let Ok(g) = group {
        resp.code = 0;
        resp.status = String::from("Success");
        resp.data = Some(g);
    }
    Ok(resp)
}

pub fn add_stewarship_group(
    db: web::Data<Pool>,
    item: web::Json<InputStewardgroup>,
) -> Result<GenericResponse<()>, diesel::result::Error> {
    let mut resp = GenericResponse::default_error("Steward group creation failed");
    let conn = db.get().unwrap();
    let leaderinfo = members.find(item.leader_id.unwrap()).first::<Member>(&conn);
    if let Err(_) = leaderinfo {
        resp.status = String::from("Group leader not found");
        return Ok(resp);
    }
    let leaderinfo = leaderinfo.unwrap();
    let leadername = match &leaderinfo.othernames {
        Some(n) => format!(
            "{} {} {}",
            (leaderinfo).firstname,
            n,
            (leaderinfo).surname
        ),
        None => format!("{} {}", (&leaderinfo).firstname, (&leaderinfo).surname),
    };
    let new_group = NewGroup {
        name: &item.name.as_ref().unwrap().as_str(),
        leader: leaderinfo.id,
        date_created: chrono::Local::now().naive_local(),
        status: "ACTIVE",
        leader_name: leadername.as_str(),
    };
    let result = diesel::insert_into(stewardgroups)
        .values(&new_group)
        .execute(&conn)?;
    if result > 0 {
        resp.code = 0;
        resp.status = String::from("Success");
    }
    Ok(resp)
}
