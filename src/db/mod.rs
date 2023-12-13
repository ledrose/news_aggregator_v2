pub mod user;
use diesel::{PgConnection, r2d2::{ConnectionManager, self}, pg::Pg};
use diesel_migrations::{EmbeddedMigrations, embed_migrations, MigrationHarness};
use dotenv::dotenv;
use anyhow::{Result,Ok};
pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Result<DBPool> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL should be in .env");
    let conn_manager = ConnectionManager::<PgConnection>::new(db_url);
    let conn_pool = r2d2::Pool::builder()
        .build(conn_manager)
        .expect("Expected to crete pool");
    Ok(conn_pool)
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migrations(conn: &DBPool) -> Result<()> {
    conn.get().unwrap()
        .run_pending_migrations(MIGRATIONS).unwrap();
    Ok(())
}