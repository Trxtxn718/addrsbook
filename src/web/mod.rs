use axum::Router;
use crate::db::DbPool;
mod prefix_handler;

pub fn router() -> Router<DbPool>{
    Router::new()
       .merge(prefix_handler::routes())
}