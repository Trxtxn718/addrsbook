use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Baut den Connection-Pool aus der Datenbank-URL.
pub fn create_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(8)
        .build(manager)
        .expect("Connection-Pool konnte nicht erstellt werden")
}

/// Führt eine blockierende Diesel-Funktion auf einem eigenen Thread aus.
pub async fn with_conn<T, F>(pool: DbPool, f: F) -> T
where
    F: FnOnce(&mut PgConnection) -> T + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().expect("Keine Datenbankverbindung verfügbar");
        f(&mut conn)
    })
    .await
    .expect("Datenbank-Task abgebrochen")
}