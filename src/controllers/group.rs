use serde_json::json;
use axum::{Router, extract::{Path, State}, http::StatusCode, routing::{delete, get, patch, post, put}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ModelTrait, EntityTrait, Set, IntoActiveModel, ConnectionTrait};
use sea_orm::EntityLoaderTrait;
use sea_orm::prelude::*;
use crate::app_state::AppState;
use crate::models::group::{ActiveModel, Entity, Model, ModelEx, Status};
use super::{source::Source as Source, };
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Group {
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub description: Option<String>,
    pub discord_url: Option<String>,
    pub founded_date: Option<String>,
    pub language: Option<String>,
    pub logo_url: Option<String>,
    pub member_count: Option<i8>,
    pub name: String,
    pub patreon_url: Option<String>,
    pub sources: Option<Vec<Source>>,
    pub status: Option<Status>,
    pub website: Option<String>
    
}

impl From<Model> for Group {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            description: model.description,
            discord_url: model.discord_url,
            founded_date: model.founded_date,
            language: model.language,
            logo_url: model.logo_url,
            member_count: model.member_count,
            name: model.name,
            patreon_url: model.patreon_url,
            sources: vec![].into(),
            status: model.status,
            website: model.website
            
        }
    }
}

impl From<ModelEx> for Group {
    fn from(model: ModelEx) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            description: model.description,
            discord_url: model.discord_url,
            founded_date: model.founded_date,
            language: model.language,
            logo_url: model.logo_url,
            member_count: model.member_count,
            name: model.name,
            patreon_url: model.patreon_url,
            sources: Some(model.sources.into_iter().map(Source::from).collect()),
            status: model.status,
            website: model.website,
            
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupCreate {
    pub description: Option<String>,
    pub discord_url: Option<String>,
    pub founded_date: Option<String>,
    pub language: Option<String>,
    pub logo_url: Option<String>,
    pub member_count: Option<i8>,
    pub name: String,
    pub patreon_url: Option<String>,
    pub sources: Option<Vec<Source>>,
    pub status: Option<Status>,
    pub website: Option<String>
    
}

impl From<GroupCreate> for ActiveModel {
    fn from(source: GroupCreate) -> Self {
        ActiveModel {
            description: Set(source.description.clone()),
            discord_url: Set(source.discord_url.clone()),
            founded_date: Set(source.founded_date.clone()),
            language: Set(source.language.clone()),
            logo_url: Set(source.logo_url.clone()),
            member_count: Set(source.member_count.clone()),
            name: Set(source.name.clone()),
            patreon_url: Set(source.patreon_url.clone()),
            status: Set(source.status.clone()),
            website: Set(source.website.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupUpdate {
    pub description: Option<String>,
    pub discord_url: Option<String>,
    pub founded_date: Option<String>,
    pub language: Option<String>,
    pub logo_url: Option<String>,
    pub member_count: Option<i8>,
    pub name: String,
    pub patreon_url: Option<String>,
    pub sources: Option<Vec<Source>>,
    pub status: Option<Status>,
    pub website: Option<String>
    
}

impl GroupUpdate {
    fn into_active_model(self, id:i32) -> ActiveModel {
        ActiveModel {
            id: Set(id),
            description: Set(self.description.clone()),
            discord_url: Set(self.discord_url.clone()),
            founded_date: Set(self.founded_date.clone()),
            language: Set(self.language.clone()),
            logo_url: Set(self.logo_url.clone()),
            member_count: Set(self.member_count.clone()),
            name: Set(self.name.clone()),
            patreon_url: Set(self.patreon_url.clone()),
            status: Set(self.status.clone()),
            website: Set(self.website.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupPatch {
    pub description: Option<String>,
    pub discord_url: Option<String>,
    pub founded_date: Option<String>,
    pub language: Option<String>,
    pub logo_url: Option<String>,
    pub member_count: Option<i8>,
    pub name: Option<String>,
    pub patreon_url: Option<String>,
    pub sources: Option<Vec<Source>>,
    pub status: Option<Status>,
    pub website: Option<String>
    
}

impl GroupPatch {
    pub fn patch_active_model(&self, active_model: &mut ActiveModel) {
        if self.description.is_some() {
            active_model.description = Set(self.description.clone());
        }if self.discord_url.is_some() {
            active_model.discord_url = Set(self.discord_url.clone());
        }if self.founded_date.is_some() {
            active_model.founded_date = Set(self.founded_date.clone());
        }if self.language.is_some() {
            active_model.language = Set(self.language.clone());
        }if self.logo_url.is_some() {
            active_model.logo_url = Set(self.logo_url.clone());
        }if self.member_count.is_some() {
            active_model.member_count = Set(self.member_count.clone());
        }if let Some(value) = &self.name {
            active_model.name = Set(value.clone());
        }if self.patreon_url.is_some() {
            active_model.patreon_url = Set(self.patreon_url.clone());
        }if self.status.is_some() {
            active_model.status = Set(self.status.clone());
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
        .with(crate::models::source::Entity)
        .all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    let responses: Vec<Group> = models.into_iter().map(Into::into).collect();

    Ok(Json(responses))
}

pub async fn create(state: State<AppState>, Json(create): Json<GroupCreate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let active_model:ActiveModel = create.into();
    let model = active_model.insert(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to insert item": e.to_string()}))))?;
        let resp: Group = model.into();
        Ok(Json(resp))

}

pub async fn patch_one(state: State<AppState>, Path(id): Path<i32>, Json(patch): Json<GroupPatch> ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let model = load_item(&state.db, id).await?;
    let mut active_model = model.into_active_model();
    patch.patch_active_model(&mut active_model);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: Group = model.into();
    Ok(Json(resp))
}

pub async fn put_one(state: State<AppState>, Path(id): Path<i32>, Json(update): Json<GroupUpdate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = load_item(&state.db, id).await?;
    let active_model = update.into_active_model(id);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: Group = model.into();
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
        .with(crate::models::source::Entity)
        .one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "not found"}))))?;
    let resp: Group = model.into();
    Ok(Json(resp))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/groups", get(list))
        .route("/groups", post(create))
        .route("/groups/{id}", get(read_one))
        .route("/groups/{id}", delete(remove))
        .route("/groups/{id}", patch(patch_one))
        .route("/groups/{id}", put(put_one))
}