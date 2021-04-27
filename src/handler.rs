use super::Pool;
use actix_web::web;
use actix_web::{Error, HttpResponse};

use crate::{
    db::{
        add_single_user, get_all_users, get_user_by_userid, perform_login_user, update_single_user,
    },
    model::{InputUser, LoginUser},
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
