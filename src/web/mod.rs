use axum::Router;
use crate::db::DbPool;
mod prefix_handler;
mod address_handler;

pub fn router() -> Router<DbPool>{
    Router::new()
       .merge(prefix_handler::routes())
       .merge(address_handler::routes())
}