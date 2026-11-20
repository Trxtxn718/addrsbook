mod address;
mod prefix;
mod schema;
mod vlan;
mod web;
mod db;


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL fehlt");
    let pool = db::create_pool(&url);

    let app= web::router().with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}