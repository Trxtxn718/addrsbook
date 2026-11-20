use axum::{
    extract::State,
    routing::get,
    Router,
};

use crate::db::{with_conn, DbPool};
use crate::address;


pub fn routes() -> Router<DbPool>{
    Router::new()
        .route("/addresses", get(list).post(create))
}

async fn list(State(pool): State<DbPool>) -> String {
    let rows = with_conn(pool, |c| address::list(c)).await.unwrap();
    format!("{rows:?}")
}

async fn create(State(_pool): State<DbPool>) -> String {
    // kommt später: Formular auslesen, prefix::create aufrufen
    "noch nicht implementiert".into()
}