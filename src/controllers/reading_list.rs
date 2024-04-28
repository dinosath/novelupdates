use serde_json::json;
use axum::{Router, extract::{Path, State}, http::StatusCode, routing::{delete, get, patch, post, put}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ModelTrait, EntityTrait, Set, IntoActiveModel, ConnectionTrait};
use sea_orm::EntityLoaderTrait;
use sea_orm::prelude::*;
use crate::app_state::AppState;
use crate::models::reading_list::{ActiveModel, Entity, Model, ModelEx, Status};
use super::{novel::Novel as Novel, user::User as User, };
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReadingList {
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub completed_date: Option<String>,
    pub current_chapter: Option<i8>,
    pub last_read: Option<String>,
    pub notes: Option<String>,
    pub novel: Vec<Novel>,
    pub personal_rating: Option<String>,
    pub started_date: Option<String>,
    pub status: Status,
    pub user: User
    
}

impl From<Model> for ReadingList {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            completed_date: model.completed_date,
            current_chapter: model.current_chapter,
            last_read: model.last_read,
            notes: model.notes,
            novel: vec![].into(),
            personal_rating: model.personal_rating,
            started_date: model.started_date,
            status: model.status,
            user: User::default()
            
        }
    }
}

impl From<ModelEx> for ReadingList {
    fn from(model: ModelEx) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            completed_date: model.completed_date,
            current_chapter: model.current_chapter,
            last_read: model.last_read,
            notes: model.notes,
            novel: model.novel.into_iter().map(Novel::from).collect(),
            personal_rating: model.personal_rating,
            started_date: model.started_date,
            status: model.status,
            user: model.user.into_option().map(Into::into).unwrap_or_default(),
            
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadingListCreate {
    pub completed_date: Option<String>,
    pub current_chapter: Option<i8>,
    pub last_read: Option<String>,
    pub notes: Option<String>,
    pub novel: Vec<Novel>,
    pub personal_rating: Option<String>,
    pub started_date: Option<String>,
    pub status: Status,
    pub user: User
    
}

impl From<ReadingListCreate> for ActiveModel {
    fn from(source: ReadingListCreate) -> Self {
        ActiveModel {
            completed_date: Set(source.completed_date.clone()),
            current_chapter: Set(source.current_chapter.clone()),
            last_read: Set(source.last_read.clone()),
            notes: Set(source.notes.clone()),
            personal_rating: Set(source.personal_rating.clone()),
            started_date: Set(source.started_date.clone()),
            status: Set(source.status.clone()),
            user_id: Set(source.user.id.clone()),..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadingListUpdate {
    pub completed_date: Option<String>,
    pub current_chapter: Option<i8>,
    pub last_read: Option<String>,
    pub notes: Option<String>,
    pub novel: Vec<Novel>,
    pub personal_rating: Option<String>,
    pub started_date: Option<String>,
    pub status: Status,
    pub user: User
    
}

impl ReadingListUpdate {
    fn into_active_model(self, id:i32) -> ActiveModel {
        ActiveModel {
            id: Set(id),
            completed_date: Set(self.completed_date.clone()),
            current_chapter: Set(self.current_chapter.clone()),
            last_read: Set(self.last_read.clone()),
            notes: Set(self.notes.clone()),
            personal_rating: Set(self.personal_rating.clone()),
            started_date: Set(self.started_date.clone()),
            status: Set(self.status.clone()),
            user_id: Set(self.user.id.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadingListPatch {
    pub completed_date: Option<String>,
    pub current_chapter: Option<i8>,
    pub last_read: Option<String>,
    pub notes: Option<String>,
    pub novel: Option<Vec<Novel>>,
    pub personal_rating: Option<String>,
    pub started_date: Option<String>,
    pub status: Option<Status>,
    pub user: Option<User>
    
}

impl ReadingListPatch {
    pub fn patch_active_model(&self, active_model: &mut ActiveModel) {
        if self.completed_date.is_some() {
            active_model.completed_date = Set(self.completed_date.clone());
        }if self.current_chapter.is_some() {
            active_model.current_chapter = Set(self.current_chapter.clone());
        }if self.last_read.is_some() {
            active_model.last_read = Set(self.last_read.clone());
        }if self.notes.is_some() {
            active_model.notes = Set(self.notes.clone());
        }if self.personal_rating.is_some() {
            active_model.personal_rating = Set(self.personal_rating.clone());
        }if self.started_date.is_some() {
            active_model.started_date = Set(self.started_date.clone());
        }if let Some(value) = &self.status {
            active_model.status = Set(value.clone());
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
    let responses: Vec<ReadingList> = models.into_iter().map(Into::into).collect();

    Ok(Json(responses))
}

pub async fn create(state: State<AppState>, Json(create): Json<ReadingListCreate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let active_model:ActiveModel = create.into();
    let model = active_model.insert(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to insert item": e.to_string()}))))?;
        let resp: ReadingList = model.into();
        Ok(Json(resp))

}

pub async fn patch_one(state: State<AppState>, Path(id): Path<i32>, Json(patch): Json<ReadingListPatch> ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let model = load_item(&state.db, id).await?;
    let mut active_model = model.into_active_model();
    patch.patch_active_model(&mut active_model);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: ReadingList = model.into();
    Ok(Json(resp))
}

pub async fn put_one(state: State<AppState>, Path(id): Path<i32>, Json(update): Json<ReadingListUpdate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = load_item(&state.db, id).await?;
    let active_model = update.into_active_model(id);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: ReadingList = model.into();
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
    let resp: ReadingList = model.into();
    Ok(Json(resp))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/reading-lists", get(list))
        .route("/reading-lists", post(create))
        .route("/reading-lists/{id}", get(read_one))
        .route("/reading-lists/{id}", delete(remove))
        .route("/reading-lists/{id}", patch(patch_one))
        .route("/reading-lists/{id}", put(put_one))
}