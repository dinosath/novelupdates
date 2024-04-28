use serde_json::json;
use axum::{Router, extract::{Path, State}, http::StatusCode, routing::{delete, get, patch, post, put}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ModelTrait, EntityTrait, Set, IntoActiveModel, ConnectionTrait};
use sea_orm::EntityLoaderTrait;
use sea_orm::prelude::*;
use crate::app_state::AppState;
use crate::models::review::{ActiveModel, Entity, Model, ModelEx, };
use super::{novel::Novel as Novel, user::User as User, };
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Review {
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub content: String,
    pub helpful_count: Option<i8>,
    pub novel: Novel,
    pub rating: String,
    pub spoiler: Option<bool>,
    pub title: Option<String>,
    pub user: User
    
}

impl From<Model> for Review {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            content: model.content,
            helpful_count: model.helpful_count,
            novel: Novel::default(),
            rating: model.rating,
            spoiler: model.spoiler,
            title: model.title,
            user: User::default()
            
        }
    }
}

impl From<ModelEx> for Review {
    fn from(model: ModelEx) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            content: model.content,
            helpful_count: model.helpful_count,
            novel: model.novel.into_option().map(Into::into).unwrap_or_default(),
            rating: model.rating,
            spoiler: model.spoiler,
            title: model.title,
            user: model.user.into_option().map(Into::into).unwrap_or_default(),
            
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReviewCreate {
    pub content: String,
    pub helpful_count: Option<i8>,
    pub novel: Novel,
    pub rating: String,
    pub spoiler: Option<bool>,
    pub title: Option<String>,
    pub user: User
    
}

impl From<ReviewCreate> for ActiveModel {
    fn from(source: ReviewCreate) -> Self {
        ActiveModel {
            content: Set(source.content.clone()),
            helpful_count: Set(source.helpful_count.clone()),
            novel_id: Set(source.novel.id.clone()),rating: Set(source.rating.clone()),
            spoiler: Set(source.spoiler.clone()),
            title: Set(source.title.clone()),
            user_id: Set(source.user.id.clone()),..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReviewUpdate {
    pub content: String,
    pub helpful_count: Option<i8>,
    pub novel: Novel,
    pub rating: String,
    pub spoiler: Option<bool>,
    pub title: Option<String>,
    pub user: User
    
}

impl ReviewUpdate {
    fn into_active_model(self, id:i32) -> ActiveModel {
        ActiveModel {
            id: Set(id),
            content: Set(self.content.clone()),
            helpful_count: Set(self.helpful_count.clone()),
            novel_id: Set(self.novel.id.clone()),
            rating: Set(self.rating.clone()),
            spoiler: Set(self.spoiler.clone()),
            title: Set(self.title.clone()),
            user_id: Set(self.user.id.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReviewPatch {
    pub content: Option<String>,
    pub helpful_count: Option<i8>,
    pub novel: Option<Novel>,
    pub rating: Option<String>,
    pub spoiler: Option<bool>,
    pub title: Option<String>,
    pub user: Option<User>
    
}

impl ReviewPatch {
    pub fn patch_active_model(&self, active_model: &mut ActiveModel) {
        if let Some(value) = &self.content {
            active_model.content = Set(value.clone());
        }if self.helpful_count.is_some() {
            active_model.helpful_count = Set(self.helpful_count.clone());
        }if let Some(value) = &self.novel {
            active_model.novel_id = Set(value.id.clone());
        }if let Some(value) = &self.rating {
            active_model.rating = Set(value.clone());
        }if self.spoiler.is_some() {
            active_model.spoiler = Set(self.spoiler.clone());
        }if self.title.is_some() {
            active_model.title = Set(self.title.clone());
        }if let Some(value) = &self.user {
            active_model.user_id = Set(value.id.clone());
        }
    }
}

async fn load_item<C>(
    db: &C,
    id: i32,
) -> Result<Model, (StatusCode, Json<serde_json::Value>)>
where
    C: ConnectionTrait,
{
    Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "not found"}))))
}

pub async fn list(state: State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let models = Entity::load()
        .with(crate::models::novel::Entity)
        .with(crate::models::user::Entity)
        .all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    let responses: Vec<Review> = models.into_iter().map(Into::into).collect();

    Ok(Json(responses))
}

pub async fn create(state: State<AppState>, Json(create): Json<ReviewCreate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let active_model:ActiveModel = create.into();
    let model = active_model.insert(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to insert item": e.to_string()}))))?;
        let resp: Review = model.into();
        Ok(Json(resp))

}

pub async fn patch_one(state: State<AppState>, Path(id): Path<i32>, Json(patch): Json<ReviewPatch> ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let model = load_item(&state.db, id).await?;
    let mut active_model = model.into_active_model();
    patch.patch_active_model(&mut active_model);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: Review = model.into();
    Ok(Json(resp))
}

pub async fn put_one(state: State<AppState>, Path(id): Path<i32>, Json(update): Json<ReviewUpdate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = load_item(&state.db, id).await?;
    let active_model = update.into_active_model(id);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: Review = model.into();
    Ok(Json(resp))
}

pub async fn remove(state: State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let model = load_item(&state.db, id).await?;
    model.delete(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn read_one(state: State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let model = Entity::load()
        .filter_by_id(id)
        .with(crate::models::novel::Entity)
        .with(crate::models::user::Entity)
        .one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "not found"}))))?;
    let resp: Review = model.into();
    Ok(Json(resp))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/reviews", get(list))
        .route("/reviews", post(create))
        .route("/reviews/{id}", get(read_one))
        .route("/reviews/{id}", delete(remove))
        .route("/reviews/{id}", patch(patch_one))
        .route("/reviews/{id}", put(put_one))
}