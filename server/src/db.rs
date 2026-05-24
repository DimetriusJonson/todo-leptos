use app::common::DbPool;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};

/*
#[cfg(feature = "ssr")]
pub async fn create_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("no database url specify");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .min_connections(1)
        .max_connections(3)
        .connect(database_url.as_str())
        .await
        .expect("could not connect to database_url");

    sqlx::migrate!("./migrations/postgres")
        .run(&pool)
        .await
        .expect("migrations failed");

    pool
}
 */

pub async fn create_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("no database url specify");
    println!("database_url={}", database_url);
    if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
        println!("Creating database {}", database_url);
        match Sqlite::create_database(&database_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = SqlitePool::connect(&database_url).await.unwrap();

    /*sqlx::migrate!("./../migrations/sqlite")
    .run(&db)
    .await
    .expect("migrations failed");*/

    db
}
