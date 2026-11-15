mod address;
mod prefix;
mod schema;
mod vlan;
use diesel::{PgConnection, pg::Pg, r2d2::{ConnectionManager, Pool}};
use ipnet::IpNet;
use axum::{routing::get, Router, extract::State};

type DbPool = Pool<ConnectionManager<PgConnection>>;


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let manager = ConnectionManager::<PgConnection>::new(std::env::var("DATABASE_URL").expect("DATABASE_URL is missing!"));
    let pool: DbPool = Pool::builder().build(manager).unwrap();
    

    let app = Router::new()
        .route("/", get(|| async { "Server is running" }))
        .route("/prefixes", get(list_prefixes))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}




async fn list_prefixes(State(pool): State<DbPool>) -> String {
    let rows = tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().unwrap();
        prefix::list(&mut conn)
    })
    .await
    .unwrap()
    .unwrap();

    format!("{rows:?}")
}
