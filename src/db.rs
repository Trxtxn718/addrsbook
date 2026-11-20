use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(8)
        .build(manager)
        .expect("connection pool could not be created")
}

pub async fn with_conn<T, F>(pool: DbPool, f: F) -> T
where
    F: FnOnce(&mut PgConnection) -> T + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().expect("No database available");
        f(&mut conn)
    })
    .await
    .expect("DB canceled")
}