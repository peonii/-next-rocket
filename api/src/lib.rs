use diesel::{PgConnection, r2d2::{ConnectionManager, Pool}};

pub mod schema;
pub mod models;
pub mod routes;

pub struct DbPool {
    pub pool: Pool<ConnectionManager<PgConnection>>
}