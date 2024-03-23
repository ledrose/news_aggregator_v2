use deadpool_diesel::postgres::{Manager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use diesel_migrations::MigrationHarness;

use dotenv::dotenv;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool,
    pub env: Config
}


const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub async fn establish_connection() -> AppState {
    let config = Config::init();
    let manager = Manager::new(
        &config.database_url, deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();
    let conn = pool.get().await.unwrap();
    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ())).await.unwrap().unwrap();
    tracing::info!("Finished pending migrations");
    AppState {db: pool,env: config}
}


#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}


impl Config {
    pub fn init() -> Config {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        Config {
            database_url,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
        }
    }
}