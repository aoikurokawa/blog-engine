use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Pool {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix-web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    pool
}
