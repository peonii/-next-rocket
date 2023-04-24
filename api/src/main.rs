use std::env;

use anyhow::Result;
use diesel::{PgConnection, r2d2::{ConnectionManager, Pool}};
use dotenvy::dotenv;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders};
use todo_app::{DbPool};
use todo_app::routes::index;

#[macro_use] extern crate rocket;

pub fn connect() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABSE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create DB pool.");

    pool
}

#[rocket::main]
async fn main() -> Result<()> {
    let methods = vec![Method::Get, Method::Post, Method::Patch, Method::Delete];

    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: methods.into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    let _ = rocket::build()
        .manage(DbPool { pool: connect() })
        .mount("/api/tasks", routes![index::view, index::add, index::complete, index::delete])
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}