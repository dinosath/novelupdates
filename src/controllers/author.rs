use serde_json::json;
use axum::{Router, extract::{Path, State}, http::StatusCode, routing::{delete, get, patch, post, put}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ModelTrait, EntityTrait, Set, IntoActiveModel, ConnectionTrait};
use sea_orm::EntityLoaderTrait;
use sea_orm::prelude::*;
use crate::app_state::AppState;
use crate::models::author::{ActiveModel, Entity, Model, ModelEx, };
use super::{novel::Novel as Novel, };
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Author {
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub bio: Option<String>,
    pub country: Option<String>,
    pub name: String,
    pub native_name: Option<String>,
    pub novels: Option<Vec<Novel>>,
    pub website: Option<String>
    
}

impl From<Model> for Author {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            bio: model.bio,
            country: model.country,
            name: model.name,
            native_name: model.native_name,
            novels: None,
            website: model.website
            
        }
    }
}

impl From<ModelEx> for Author {
    fn from(model: ModelEx) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            bio: model.bio,
            country: model.country,
            name: model.name,
            native_name: model.native_name,
            novels: Some(model.novels.into_iter().map(Novel::from).collect()),
            website: model.website,
            
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorCreate {
    pub bio: Option<String>,
    pub country: Option<String>,
    pub name: String,
    pub native_name: Option<String>,
    pub novels: Option<Vec<Novel>>,
    pub website: Option<String>
    
}

impl From<AuthorCreate> for ActiveModel {
    fn from(source: AuthorCreate) -> Self {
        ActiveModel {
            bio: Set(source.bio.clone()),
            country: Set(source.country.clone()),
            name: Set(source.name.clone()),
            native_name: Set(source.native_name.clone()),
            website: Set(source.website.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorUpdate {
    pub bio: Option<String>,
    pub country: Option<String>,
    pub name: String,
    pub native_name: Option<String>,
    pub novels: Option<Vec<Novel>>,
    pub website: Option<String>
    
}

impl AuthorUpdate {
    fn into_active_model(self, id:i32) -> ActiveModel {
        ActiveModel {
            id: Set(id),
            bio: Set(self.bio.clone()),
            country: Set(self.country.clone()),
            name: Set(self.name.clone()),
            native_name: Set(self.native_name.clone()),
            website: Set(self.website.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorPatch {
    pub bio: Option<String>,
    pub country: Option<String>,
    pub name: Option<String>,
    pub native_name: Option<String>,
    pub novels: Option<Vec<Novel>>,
    pub website: Option<String>
    
}

impl AuthorPatch {
    pub fn patch_active_model(&self, active_model: &mut ActiveModel) {
        if self.bio.is_some() {
            active_model.bio = Set(self.bio.clone());
        }if self.country.is_some() {
            active_model.country = Set(self.country.clone());
        }if let Some(value) = &self.name {
            active_model.name = Set(value.clone());
        }if self.native_name.is_some() {
            active_model.native_name = Set(self.native_name.clone());
        }if self.website.is_some() {
            active_model.website = Set(self.website.clone());
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
    let responses: Vec<Author> = models.into_iter().map(Into::into).collect();

    Ok(Json(responses))
}

pub async fn create(state: State<AppState>, Json(create): Json<AuthorCreate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let active_model:ActiveModel = create.into();
    let model = active_model.insert(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to insert item": e.to_string()}))))?;
        let resp: Author = model.into();
        Ok(Json(resp))

}

pub async fn patch_one(state: State<AppState>, Path(id): Path<i32>, Json(patch): Json<AuthorPatch> ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let model = load_item(&state.db, id).await?;
    let mut active_model = model.into_active_model();
    patch.patch_active_model(&mut active_model);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: Author = model.into();
    Ok(Json(resp))
}

pub async fn put_one(state: State<AppState>, Path(id): Path<i32>, Json(update): Json<AuthorUpdate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = load_item(&state.db, id).await?;
    let active_model = update.into_active_model(id);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: Author = model.into();
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
    let resp: Author = model.into();
    Ok(Json(resp))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/authors", get(list))
        .route("/authors", post(create))
        .route("/authors/{id}", get(read_one))
        .route("/authors/{id}", delete(remove))
        .route("/authors/{id}", patch(patch_one))
        .route("/authors/{id}", put(put_one))
}