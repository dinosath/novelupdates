use serde_json::json;
use axum::{Router, extract::{Path, State}, http::StatusCode, routing::{delete, get, patch, post, put}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ModelTrait, EntityTrait, Set, IntoActiveModel, ConnectionTrait};
use sea_orm::EntityLoaderTrait;
use sea_orm::prelude::*;
use crate::app_state::AppState;
use crate::models::tag::{ActiveModel, Entity, Model, ModelEx, Category};
use super::{novel::Novel as Novel, };
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Tag {
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub category: Option<Category>,
    pub description: Option<String>,
    pub name: String,
    pub novels: Option<Vec<Novel>>,
    pub slug: String,
    pub usage_count: Option<i8>
    
}

impl From<Model> for Tag {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            category: model.category,
            description: model.description,
            name: model.name,
            novels: None,
            slug: model.slug,
            usage_count: model.usage_count
            
        }
    }
}

impl From<ModelEx> for Tag {
    fn from(model: ModelEx) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            category: model.category,
            description: model.description,
            name: model.name,
            novels: Some(model.novels.into_iter().map(Novel::from).collect()),
            slug: model.slug,
            usage_count: model.usage_count,
            
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagCreate {
    pub category: Option<Category>,
    pub description: Option<String>,
    pub name: String,
    pub novels: Option<Vec<Novel>>,
    pub slug: String,
    pub usage_count: Option<i8>
    
}

impl From<TagCreate> for ActiveModel {
    fn from(source: TagCreate) -> Self {
        ActiveModel {
            category: Set(source.category.clone()),
            description: Set(source.description.clone()),
            name: Set(source.name.clone()),
            slug: Set(source.slug.clone()),
            usage_count: Set(source.usage_count.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagUpdate {
    pub category: Option<Category>,
    pub description: Option<String>,
    pub name: String,
    pub novels: Option<Vec<Novel>>,
    pub slug: String,
    pub usage_count: Option<i8>
    
}

impl TagUpdate {
    fn into_active_model(self, id:i32) -> ActiveModel {
        ActiveModel {
            id: Set(id),
            category: Set(self.category.clone()),
            description: Set(self.description.clone()),
            name: Set(self.name.clone()),
            slug: Set(self.slug.clone()),
            usage_count: Set(self.usage_count.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagPatch {
    pub category: Option<Category>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub novels: Option<Vec<Novel>>,
    pub slug: Option<String>,
    pub usage_count: Option<i8>
    
}

impl TagPatch {
    pub fn patch_active_model(&self, active_model: &mut ActiveModel) {
        if self.category.is_some() {
            active_model.category = Set(self.category.clone());
        }if self.description.is_some() {
            active_model.description = Set(self.description.clone());
        }if let Some(value) = &self.name {
            active_model.name = Set(value.clone());
        }if let Some(value) = &self.slug {
            active_model.slug = Set(value.clone());
        }if self.usage_count.is_some() {
            active_model.usage_count = Set(self.usage_count.clone());
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
        .all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    let responses: Vec<Tag> = models.into_iter().map(Into::into).collect();

    Ok(Json(responses))
}

pub async fn create(state: State<AppState>, Json(create): Json<TagCreate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let active_model:ActiveModel = create.into();
    let model = active_model.insert(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to insert item": e.to_string()}))))?;
        let resp: Tag = model.into();
        Ok(Json(resp))

}

pub async fn patch_one(state: State<AppState>, Path(id): Path<i32>, Json(patch): Json<TagPatch> ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let model = load_item(&state.db, id).await?;
    let mut active_model = model.into_active_model();
    patch.patch_active_model(&mut active_model);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: Tag = model.into();
    Ok(Json(resp))
}

pub async fn put_one(state: State<AppState>, Path(id): Path<i32>, Json(update): Json<TagUpdate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = load_item(&state.db, id).await?;
    let active_model = update.into_active_model(id);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: Tag = model.into();
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
        .one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "not found"}))))?;
    let resp: Tag = model.into();
    Ok(Json(resp))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/tags", get(list))
        .route("/tags", post(create))
        .route("/tags/{id}", get(read_one))
        .route("/tags/{id}", delete(remove))
        .route("/tags/{id}", patch(patch_one))
        .route("/tags/{id}", put(put_one))
}