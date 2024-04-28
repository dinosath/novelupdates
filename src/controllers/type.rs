use serde_json::json;
use axum::{Router, extract::{Path, State}, http::StatusCode, routing::{delete, get, patch, post, put}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ModelTrait, EntityTrait, Set, IntoActiveModel, ConnectionTrait};
use sea_orm::EntityLoaderTrait;
use sea_orm::prelude::*;
use crate::app_state::AppState;
use crate::models::r#type::{ActiveModel, Entity, Model, ModelEx, };
use super::{novel::Novel as Novel, };
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Type {
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub description: Option<String>,
    pub name: String,
    pub novels: Option<Vec<Novel>>
    
}

impl From<Model> for Type {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            description: model.description,
            name: model.name,
            novels: vec![].into()
            
        }
    }
}

impl From<ModelEx> for Type {
    fn from(model: ModelEx) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            description: model.description,
            name: model.name,
            novels: Some(model.novels.into_iter().map(Novel::from).collect()),
            
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypeCreate {
    pub description: Option<String>,
    pub name: String,
    pub novels: Option<Vec<Novel>>
    
}

impl From<TypeCreate> for ActiveModel {
    fn from(source: TypeCreate) -> Self {
        ActiveModel {
            description: Set(source.description.clone()),
            name: Set(source.name.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypeUpdate {
    pub description: Option<String>,
    pub name: String,
    pub novels: Option<Vec<Novel>>
    
}

impl TypeUpdate {
    fn into_active_model(self, id:i32) -> ActiveModel {
        ActiveModel {
            id: Set(id),
            description: Set(self.description.clone()),
            name: Set(self.name.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypePatch {
    pub description: Option<String>,
    pub name: Option<String>,
    pub novels: Option<Vec<Novel>>
    
}

impl TypePatch {
    pub fn patch_active_model(&self, active_model: &mut ActiveModel) {
        if self.description.is_some() {
            active_model.description = Set(self.description.clone());
        }if let Some(value) = &self.name {
            active_model.name = Set(value.clone());
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
    let responses: Vec<Type> = models.into_iter().map(Into::into).collect();

    Ok(Json(responses))
}

pub async fn create(state: State<AppState>, Json(create): Json<TypeCreate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let active_model:ActiveModel = create.into();
    let model = active_model.insert(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to insert item": e.to_string()}))))?;
        let resp: Type = model.into();
        Ok(Json(resp))

}

pub async fn patch_one(state: State<AppState>, Path(id): Path<i32>, Json(patch): Json<TypePatch> ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let model = load_item(&state.db, id).await?;
    let mut active_model = model.into_active_model();
    patch.patch_active_model(&mut active_model);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: Type = model.into();
    Ok(Json(resp))
}

pub async fn put_one(state: State<AppState>, Path(id): Path<i32>, Json(update): Json<TypeUpdate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = load_item(&state.db, id).await?;
    let active_model = update.into_active_model(id);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: Type = model.into();
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
    let resp: Type = model.into();
    Ok(Json(resp))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/types", get(list))
        .route("/types", post(create))
        .route("/types/{id}", get(read_one))
        .route("/types/{id}", delete(remove))
        .route("/types/{id}", patch(patch_one))
        .route("/types/{id}", put(put_one))
}