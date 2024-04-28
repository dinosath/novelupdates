use serde_json::json;
use axum::{Router, extract::{Path, State}, http::StatusCode, routing::{delete, get, patch, post, put}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ModelTrait, EntityTrait, Set, IntoActiveModel, ConnectionTrait};
use sea_orm::EntityLoaderTrait;
use sea_orm::prelude::*;
use crate::app_state::AppState;
use crate::models::source::{ActiveModel, Entity, Model, ModelEx, Status};
use super::{chapter::Chapter as Chapter, group::Group as Group, novel::Novel as Novel, };
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Source {
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub chapter_count: Option<i8>,
    pub chapters: Option<Vec<Chapter>>,
    pub completely_translated: Option<bool>,
    pub group: Option<Group>,
    pub is_official: Option<bool>,
    pub language: String,
    pub name: String,
    pub novel: Novel,
    pub status: Option<Status>,
    pub url: Option<String>
    
}

impl From<Model> for Source {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            chapter_count: model.chapter_count,
            chapters: vec![].into(),
            completely_translated: model.completely_translated,
            group: None,
            is_official: model.is_official,
            language: model.language,
            name: model.name,
            novel: Novel::default(),
            status: model.status,
            url: model.url
            
        }
    }
}

impl From<ModelEx> for Source {
    fn from(model: ModelEx) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            chapter_count: model.chapter_count,
            chapters: Some(model.chapters.into_iter().map(Chapter::from).collect()),
            completely_translated: model.completely_translated,
            group: model.group.into_option().map(Into::into),
            is_official: model.is_official,
            language: model.language,
            name: model.name,
            novel: model.novel.into_option().map(Into::into).unwrap_or_default(),
            status: model.status,
            url: model.url,
            
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceCreate {
    pub chapter_count: Option<i8>,
    pub chapters: Option<Vec<Chapter>>,
    pub completely_translated: Option<bool>,
    pub group: Option<Group>,
    pub is_official: Option<bool>,
    pub language: String,
    pub name: String,
    pub novel: Novel,
    pub status: Option<Status>,
    pub url: Option<String>
    
}

impl From<SourceCreate> for ActiveModel {
    fn from(source: SourceCreate) -> Self {
        ActiveModel {
            chapter_count: Set(source.chapter_count.clone()),
            completely_translated: Set(source.completely_translated.clone()),
            group_id: Set(source.group.unwrap_or_default().id.clone()),is_official: Set(source.is_official.clone()),
            language: Set(source.language.clone()),
            name: Set(source.name.clone()),
            novel_id: Set(source.novel.id.clone()),status: Set(source.status.clone()),
            url: Set(source.url.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceUpdate {
    pub chapter_count: Option<i8>,
    pub chapters: Option<Vec<Chapter>>,
    pub completely_translated: Option<bool>,
    pub group: Option<Group>,
    pub is_official: Option<bool>,
    pub language: String,
    pub name: String,
    pub novel: Novel,
    pub status: Option<Status>,
    pub url: Option<String>
    
}

impl SourceUpdate {
    fn into_active_model(self, id:i32) -> ActiveModel {
        ActiveModel {
            id: Set(id),
            chapter_count: Set(self.chapter_count.clone()),
            completely_translated: Set(self.completely_translated.clone()),
            group_id: Set(self.group.unwrap_or_default().id.clone()),
            is_official: Set(self.is_official.clone()),
            language: Set(self.language.clone()),
            name: Set(self.name.clone()),
            novel_id: Set(self.novel.id.clone()),
            status: Set(self.status.clone()),
            url: Set(self.url.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourcePatch {
    pub chapter_count: Option<i8>,
    pub chapters: Option<Vec<Chapter>>,
    pub completely_translated: Option<bool>,
    pub group: Option<Group>,
    pub is_official: Option<bool>,
    pub language: Option<String>,
    pub name: Option<String>,
    pub novel: Option<Novel>,
    pub status: Option<Status>,
    pub url: Option<String>
    
}

impl SourcePatch {
    pub fn patch_active_model(&self, active_model: &mut ActiveModel) {
        if self.chapter_count.is_some() {
            active_model.chapter_count = Set(self.chapter_count.clone());
        }if self.completely_translated.is_some() {
            active_model.completely_translated = Set(self.completely_translated.clone());
        }if let Some(value) = &self.group {
            active_model.group_id = Set(value.id.clone());
        }if self.is_official.is_some() {
            active_model.is_official = Set(self.is_official.clone());
        }if let Some(value) = &self.language {
            active_model.language = Set(value.clone());
        }if let Some(value) = &self.name {
            active_model.name = Set(value.clone());
        }if let Some(value) = &self.novel {
            active_model.novel_id = Set(value.id.clone());
        }if self.status.is_some() {
            active_model.status = Set(self.status.clone());
        }if self.url.is_some() {
            active_model.url = Set(self.url.clone());
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
        .with(crate::models::chapter::Entity)
        .with(crate::models::group::Entity)
        .with(crate::models::novel::Entity)
        .all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    let responses: Vec<Source> = models.into_iter().map(Into::into).collect();

    Ok(Json(responses))
}

pub async fn create(state: State<AppState>, Json(create): Json<SourceCreate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let active_model:ActiveModel = create.into();
    let model = active_model.insert(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to insert item": e.to_string()}))))?;
        let resp: Source = model.into();
        Ok(Json(resp))

}

pub async fn patch_one(state: State<AppState>, Path(id): Path<i32>, Json(patch): Json<SourcePatch> ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let model = load_item(&state.db, id).await?;
    let mut active_model = model.into_active_model();
    patch.patch_active_model(&mut active_model);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: Source = model.into();
    Ok(Json(resp))
}

pub async fn put_one(state: State<AppState>, Path(id): Path<i32>, Json(update): Json<SourceUpdate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = load_item(&state.db, id).await?;
    let active_model = update.into_active_model(id);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: Source = model.into();
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
        .with(crate::models::chapter::Entity)
        .with(crate::models::group::Entity)
        .with(crate::models::novel::Entity)
        .one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "not found"}))))?;
    let resp: Source = model.into();
    Ok(Json(resp))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/sources", get(list))
        .route("/sources", post(create))
        .route("/sources/{id}", get(read_one))
        .route("/sources/{id}", delete(remove))
        .route("/sources/{id}", patch(patch_one))
        .route("/sources/{id}", put(put_one))
}