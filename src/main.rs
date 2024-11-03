#[macro_use]
extern crate rocket;

use fairings::cors::{options, CORS};
use migrator::Migrator;
use sea_orm_migration::prelude::*;

mod auth;
mod controllers;
mod db;
mod entities;
mod fairings;
mod migrator;

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
    jwt_secret: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_host: std::env::var("BOOKSTORE_DB_HOST").expect("Please set the BOOKSTORE_DB_HOST env variable."),
            db_port: std::env::var("BOOKSTORE_DB_PORT").expect("Please set the BOOKSTORE_DB_PORT env variable."),
            db_username: std::env::var("BOOKSTORE_DB_USERNAME").expect("Please set the BOOKSTORE_DB_USERNAME env variable."),
            db_password: std::env::var("BOOKSTORE_DB_PASSWORD").expect("Please set the BOOKSTORE_DB_PASSWORD env variable."),
            db_database: std::env::var("BOOKSTORE_DB_DATABASE").expect("Please set the BOOKSTORE_DB_DATABASE env variable."),
            jwt_secret: std::env::var("BOOKSTORE_JWT_SECRET").expect("Please set the BOOKSTORE_JWT_SECRET env variable."),
        }
    }
}

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();
    let config = AppConfig::default();
    let db = db::connect(&config).await.expect("Failed to connect to the database");
    Migrator::up(&db, None).await.expect("Failed to run migrations");

    rocket::build()
        .attach(CORS)
        .manage(db)
        .manage(config)
        .mount("/", routes![options])
        .mount(
            "/auth",
            routes![
                controllers::auth::sign_in,
                controllers::auth::sign_up,
                controllers::auth::me
            ],
        )
        .mount(
            "/authors",
            routes![
                controllers::authors::index,
                controllers::authors::create,
                controllers::authors::show,
                controllers::authors::update,
                controllers::authors::delete,
                controllers::authors::get_books
            ],
        )
        .mount(
            "/books",
            routes![
                controllers::books::index,
                controllers::books::create,
                controllers::books::show,
                controllers::books::update,
                controllers::books::delete,
            ],
        )
}