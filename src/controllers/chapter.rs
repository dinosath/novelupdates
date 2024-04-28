use serde_json::json;
use axum::{Router, extract::{Path, State}, http::StatusCode, routing::{delete, get, patch, post, put}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ModelTrait, EntityTrait, Set, IntoActiveModel, ConnectionTrait};
use sea_orm::EntityLoaderTrait;
use sea_orm::prelude::*;
use crate::app_state::AppState;
use crate::models::chapter::{ActiveModel, Entity, Model, ModelEx, };
use super::{novel::Novel as Novel, source::Source as Source, };
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Chapter {
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub content_url: Option<String>,
    pub is_locked: Option<bool>,
    pub language: String,
    pub novels: Option<Vec<Novel>>,
    pub number: String,
    pub part: Option<i8>,
    pub release_date: Option<String>,
    pub source: Source,
    pub title: String,
    pub views: Option<i8>,
    pub volume: Option<i8>
    
}

impl From<Model> for Chapter {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            content_url: model.content_url,
            is_locked: model.is_locked,
            language: model.language,
            novels: None,
            number: model.number,
            part: model.part,
            release_date: model.release_date,
            source: Source::default(),
            title: model.title,
            views: model.views,
            volume: model.volume
            
        }
    }
}

impl From<ModelEx> for Chapter {
    fn from(model: ModelEx) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            content_url: model.content_url,
            is_locked: model.is_locked,
            language: model.language,
            novels: Some(model.novels.into_iter().map(Novel::from).collect()),
            number: model.number,
            part: model.part,
            release_date: model.release_date,
            source: model.source.into_option().map(Into::into).unwrap_or_default(),
            title: model.title,
            views: model.views,
            volume: model.volume,
            
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChapterCreate {
    pub content_url: Option<String>,
    pub is_locked: Option<bool>,
    pub language: String,
    pub novels: Option<Vec<Novel>>,
    pub number: String,
    pub part: Option<i8>,
    pub release_date: Option<String>,
    pub source: Source,
    pub title: String,
    pub views: Option<i8>,
    pub volume: Option<i8>
    
}

impl From<ChapterCreate> for ActiveModel {
    fn from(source: ChapterCreate) -> Self {
        ActiveModel {
            content_url: Set(source.content_url.clone()),
            is_locked: Set(source.is_locked.clone()),
            language: Set(source.language.clone()),
            number: Set(source.number.clone()),
            part: Set(source.part.clone()),
            release_date: Set(source.release_date.clone()),
            source_id: Set(source.source.id.clone()),title: Set(source.title.clone()),
            views: Set(source.views.clone()),
            volume: Set(source.volume.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChapterUpdate {
    pub content_url: Option<String>,
    pub is_locked: Option<bool>,
    pub language: String,
    pub novels: Option<Vec<Novel>>,
    pub number: String,
    pub part: Option<i8>,
    pub release_date: Option<String>,
    pub source: Source,
    pub title: String,
    pub views: Option<i8>,
    pub volume: Option<i8>
    
}

impl ChapterUpdate {
    fn into_active_model(self, id:i32) -> ActiveModel {
        ActiveModel {
            id: Set(id),
            content_url: Set(self.content_url.clone()),
            is_locked: Set(self.is_locked.clone()),
            language: Set(self.language.clone()),
            number: Set(self.number.clone()),
            part: Set(self.part.clone()),
            release_date: Set(self.release_date.clone()),
            source_id: Set(self.source.id.clone()),
            title: Set(self.title.clone()),
            views: Set(self.views.clone()),
            volume: Set(self.volume.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChapterPatch {
    pub content_url: Option<String>,
    pub is_locked: Option<bool>,
    pub language: Option<String>,
    pub novels: Option<Vec<Novel>>,
    pub number: Option<String>,
    pub part: Option<i8>,
    pub release_date: Option<String>,
    pub source: Option<Source>,
    pub title: Option<String>,
    pub views: Option<i8>,
    pub volume: Option<i8>
    
}

impl ChapterPatch {
    pub fn patch_active_model(&self, active_model: &mut ActiveModel) {
        if self.content_url.is_some() {
            active_model.content_url = Set(self.content_url.clone());
        }if self.is_locked.is_some() {
            active_model.is_locked = Set(self.is_locked.clone());
        }if let Some(value) = &self.language {
            active_model.language = Set(value.clone());
        }if let Some(value) = &self.number {
            active_model.number = Set(value.clone());
        }if self.part.is_some() {
            active_model.part = Set(self.part.clone());
        }if self.release_date.is_some() {
            active_model.release_date = Set(self.release_date.clone());
        }if let Some(value) = &self.source {
            active_model.source_id = Set(value.id.clone());
        }if let Some(value) = &self.title {
            active_model.title = Set(value.clone());
        }if self.views.is_some() {
            active_model.views = Set(self.views.clone());
        }if self.volume.is_some() {
            active_model.volume = Set(self.volume.clone());
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
        .with(crate::models::source::Entity)
        .all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    let responses: Vec<Chapter> = models.into_iter().map(Into::into).collect();

    Ok(Json(responses))
}

pub async fn create(state: State<AppState>, Json(create): Json<ChapterCreate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let active_model:ActiveModel = create.into();
    let model = active_model.insert(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to insert item": e.to_string()}))))?;
        let resp: Chapter = model.into();
        Ok(Json(resp))

}

pub async fn patch_one(state: State<AppState>, Path(id): Path<i32>, Json(patch): Json<ChapterPatch> ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let model = load_item(&state.db, id).await?;
    let mut active_model = model.into_active_model();
    patch.patch_active_model(&mut active_model);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: Chapter = model.into();
    Ok(Json(resp))
}

pub async fn put_one(state: State<AppState>, Path(id): Path<i32>, Json(update): Json<ChapterUpdate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = load_item(&state.db, id).await?;
    let active_model = update.into_active_model(id);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: Chapter = model.into();
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
        .with(crate::models::source::Entity)
        .one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "not found"}))))?;
    let resp: Chapter = model.into();
    Ok(Json(resp))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/chapters", get(list))
        .route("/chapters", post(create))
        .route("/chapters/{id}", get(read_one))
        .route("/chapters/{id}", delete(remove))
        .route("/chapters/{id}", patch(patch_one))
        .route("/chapters/{id}", put(put_one))
}