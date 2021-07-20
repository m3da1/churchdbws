#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod db;
mod handler;
mod model;
mod schema;
mod util;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix=debug");
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let domain = std::env::var("HOST_DOMAIN").expect("HOST_DOMAIN mut be set");
    println!("Setting database connectivity on {0}", database_url);
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    println!("Starting server on {0}", domain);
    HttpServer::new(move || {
        // let cors = Cors::new()
        //     .allowed_methods(vec!["POST", "PUT", "GET", "DELETE", "OPTIONS"])
        //     .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        //     .allowed_header(http::header::CONTENT_TYPE)
        //     .max_age(3600);
        App::new()
            .wrap(Cors::new().send_wildcard().finish())
            .wrap(Logger::default())
            .service(
                web::scope("churchdbws")
                    .data(pool.clone())
                    .route("/users", web::get().to(handler::get_users))
                    .route("/users/{id}", web::get().to(handler::get_user_by_id))
                    .route("/users/{id}", web::delete().to(handler::delete_user_by_id))
                    .route("/users", web::post().to(handler::add_user))
                    .route("/users", web::put().to(handler::update_user))
                    .route("/users/login", web::post().to(handler::login_user))
                    .route(
                        "/users/changepasswd",
                        web::post().to(handler::change_user_password),
                    )
                    .route("/members", web::get().to(handler::get_members))
                    .route("/members/{id}", web::get().to(handler::get_member_by_id))
                    .route("/members", web::post().to(handler::add_member))
                    .route("/members", web::put().to(handler::update_member))
                    .route("/members/{id}", web::delete().to(handler::delete_member))
                    .route("/stewardgroup", web::get().to(handler::get_steward_groups))
                    .route(
                        "/stewardgroup/{id}",
                        web::get().to(handler::get_steward_group_by_id),
                    )
                    .route("/stewardgroup", web::post().to(handler::add_steward_group))
                    .route(
                        "/stewardgroup",
                        web::put().to(handler::update_steward_group),
                    )
                    .route(
                        "/stewardgroup/{id}",
                        web::delete().to(handler::delete_steward_group),
                    ), // .route("/zonalgroup", web::get().to(handler::get_zonal_groups)))
            )
    })
    .bind(&domain)?
    .run()
    .await
}
