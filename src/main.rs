#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, middleware, web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod handler;
mod model;
mod schema;
mod db;
mod util;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start http server
    HttpServer::new(move || {
        // let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(middleware::Logger::default())
            // .wrap(auth)Ø
            .data(pool.clone())
            .route("/users", web::get().to(handler::get_users))
            .route("/users/{id}", web::get().to(handler::get_user_by_id))
            .route("/users", web::post().to(handler::add_user))
            .route("/users", web::put().to(handler::update_user))
            .route("/users/login", web::post().to(handler::login_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}