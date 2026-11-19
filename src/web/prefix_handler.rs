use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};

use crate::db::{with_conn, DbPool};
use crate::prefix;

pub fn routes() -> Router<DbPool> {
    Router::new()
        .route("/prefixes", get(list).post(create))
        .route("/prefixes/{id}", get(show))
}

async fn list(State(pool): State<DbPool>) -> String {
    let rows = with_conn(pool, |c| prefix::list(c)).await.unwrap();
    format!("{rows:?}")
}

async fn show(State(pool): State<DbPool>, Path(id): Path<i32>) -> String {
    let row = with_conn(pool, move |c| prefix::find(c, id)).await.unwrap();
    format!("{row:?}")
}

async fn create(State(_pool): State<DbPool>) -> String {
    // kommt später: Formular auslesen, prefix::create aufrufen
    "noch nicht implementiert".into()
}