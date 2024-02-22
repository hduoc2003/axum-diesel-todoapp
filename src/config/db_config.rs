use crate::api::v1::db::database::{get_connection_pool, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

pub async fn config_db() -> Pool {
    let pool = get_connection_pool().await;
    // let mut conn = pool.clone().get().await.unwrap();
    // // let embedded_migrations = embed_migrations!("migrations/");
    // let migrations = diesel_async_migrations::embed_migrations!();

    // migrations.run_pending_migrations(&mut conn).await.unwrap();
    return pool.clone();
}
