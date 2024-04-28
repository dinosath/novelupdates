

pub mod artist;
pub mod author;
pub mod chapter;
pub mod group;
pub mod novel;
pub mod publisher;
pub mod reading_list;
pub mod review;
pub mod source;
pub mod tag;
pub mod r#type;
pub mod user;
use axum::Router;
use crate::app_state::AppState;

/// Returns a Router with all entity controllers merged.
pub fn routes(prefix:&str, state:AppState) -> Router {
    let router = Router::new()
        .merge(artist::routes())
        .merge(author::routes())
        .merge(chapter::routes())
        .merge(group::routes())
        .merge(novel::routes())
        .merge(publisher::routes())
        .merge(reading_list::routes())
        .merge(review::routes())
        .merge(source::routes())
        .merge(tag::routes())
        .merge(r#type::routes())
        .merge(user::routes())
        ;
    Router::new()
        .nest(prefix, router)
        .with_state(state)
}