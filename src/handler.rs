use super::Pool;
use actix_web::web;
use actix_web::{Error, HttpResponse};

use crate::{
    db::{
        add_single_member, add_single_user, add_stewarship_group, delete_single_member,
        delete_single_steward_group, delete_single_user, get_all_members, get_all_users,
        get_members_by_userid, get_single_steward_group, get_stewardship_groups,
        get_user_by_userid, perform_login_user, update_password, update_single_member,
        update_single_steward_group, update_single_user,
    },
    model::{ChangeUserPassword, InputMember, InputStewardgroup, InputUser, LoginUser},
};

pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|users| HttpResponse::Ok().json(users))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_user_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || get_user_by_userid(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_user(db, item))
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn login_user(
    db: web::Data<Pool>,
    item: web::Json<LoginUser>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || perform_login_user(db, item))
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn update_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || update_single_user(db, item))
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn delete_user_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_user(db, user_id.into_inner()))
            .await
            .map(|resp| HttpResponse::Ok().json(resp))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

pub async fn change_user_password(
    db: web::Data<Pool>,
    item: web::Json<ChangeUserPassword>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || update_password(db, item.into_inner()))
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_members(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_members(db))
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_member_by_id(
    db: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || get_members_by_userid(db, id.into_inner()))
            .await
            .map(|resp| HttpResponse::Ok().json(resp))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

pub async fn add_member(
    db: web::Data<Pool>,
    item: web::Json<InputMember>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_member(db, item))
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn update_member(
    db: web::Data<Pool>,
    item: web::Json<InputMember>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || update_single_member(db, item))
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn delete_member(
    db: web::Data<Pool>,
    member_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || delete_single_member(db, member_id))
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_steward_groups(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_stewardship_groups(db))
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_steward_group_by_id(
    db: web::Data<Pool>,
    steward_group_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || get_single_steward_group(db, steward_group_id.into_inner()))
            .await
            .map(|resp| HttpResponse::Ok().json(resp))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

pub async fn add_steward_group(
    db: web::Data<Pool>,
    group_info: web::Json<InputStewardgroup>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_stewarship_group(db, group_info))
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn update_steward_group(
    db: web::Data<Pool>,
    group_info: web::Json<InputStewardgroup>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || update_single_steward_group(db, group_info))
            .await
            .map(|resp| HttpResponse::Ok().json(resp))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

pub async fn delete_steward_group(
    db: web::Data<Pool>,
    group_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_steward_group(db, group_id.into_inner()))
            .await
            .map(|resp| HttpResponse::Ok().json(resp))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// pub async fn get_zonal_groups(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
//     Ok(
//         web::block(move || get_all_zonal_groups(db))
//             .await
//             .map(|resp| HttpResponse::Ok().json(resp))
//             .map_err(|_| HttpResponse::InternalServerError())?,
//     )
// }
