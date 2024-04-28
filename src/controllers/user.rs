use serde_json::json;
use axum::{Router, extract::{Path, State}, http::StatusCode, routing::{delete, get, patch, post, put}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ModelTrait, EntityTrait, Set, IntoActiveModel, ConnectionTrait};
use sea_orm::EntityLoaderTrait;
use sea_orm::prelude::*;
use crate::app_state::AppState;
use crate::models::user::{ActiveModel, Entity, Model, ModelEx, };
use super::{reading_list::ReadingList as ReadingList, review::Review as Review, };
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub display_name: Option<String>,
    pub email: String,
    pub joined_date: Option<String>,
    pub last_active: Option<String>,
    pub password_hash: String,
    pub reading_lists: Option<Vec<ReadingList>>,
    pub reviews: Option<Vec<Review>>,
    pub username: String
    
}

impl From<Model> for User {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            avatar_url: model.avatar_url,
            bio: model.bio,
            display_name: model.display_name,
            email: model.email,
            joined_date: model.joined_date,
            last_active: model.last_active,
            password_hash: model.password_hash,
            reading_lists: vec![].into(),
            reviews: vec![].into(),
            username: model.username
            
        }
    }
}

impl From<ModelEx> for User {
    fn from(model: ModelEx) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            avatar_url: model.avatar_url,
            bio: model.bio,
            display_name: model.display_name,
            email: model.email,
            joined_date: model.joined_date,
            last_active: model.last_active,
            password_hash: model.password_hash,
            reading_lists: Some(model.reading_lists.into_iter().map(ReadingList::from).collect()),
            reviews: Some(model.reviews.into_iter().map(Review::from).collect()),
            username: model.username,
            
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserCreate {
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub display_name: Option<String>,
    pub email: String,
    pub joined_date: Option<String>,
    pub last_active: Option<String>,
    pub password_hash: String,
    pub reading_lists: Option<Vec<ReadingList>>,
    pub reviews: Option<Vec<Review>>,
    pub username: String
    
}

impl From<UserCreate> for ActiveModel {
    fn from(source: UserCreate) -> Self {
        ActiveModel {
            avatar_url: Set(source.avatar_url.clone()),
            bio: Set(source.bio.clone()),
            display_name: Set(source.display_name.clone()),
            email: Set(source.email.clone()),
            joined_date: Set(source.joined_date.clone()),
            last_active: Set(source.last_active.clone()),
            password_hash: Set(source.password_hash.clone()),
            username: Set(source.username.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserUpdate {
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub display_name: Option<String>,
    pub email: String,
    pub joined_date: Option<String>,
    pub last_active: Option<String>,
    pub password_hash: String,
    pub reading_lists: Option<Vec<ReadingList>>,
    pub reviews: Option<Vec<Review>>,
    pub username: String
    
}

impl UserUpdate {
    fn into_active_model(self, id:i32) -> ActiveModel {
        ActiveModel {
            id: Set(id),
            avatar_url: Set(self.avatar_url.clone()),
            bio: Set(self.bio.clone()),
            display_name: Set(self.display_name.clone()),
            email: Set(self.email.clone()),
            joined_date: Set(self.joined_date.clone()),
            last_active: Set(self.last_active.clone()),
            password_hash: Set(self.password_hash.clone()),
            username: Set(self.username.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPatch {
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub joined_date: Option<String>,
    pub last_active: Option<String>,
    pub password_hash: Option<String>,
    pub reading_lists: Option<Vec<ReadingList>>,
    pub reviews: Option<Vec<Review>>,
    pub username: Option<String>
    
}

impl UserPatch {
    pub fn patch_active_model(&self, active_model: &mut ActiveModel) {
        if self.avatar_url.is_some() {
            active_model.avatar_url = Set(self.avatar_url.clone());
        }if self.bio.is_some() {
            active_model.bio = Set(self.bio.clone());
        }if self.display_name.is_some() {
            active_model.display_name = Set(self.display_name.clone());
        }if let Some(value) = &self.email {
            active_model.email = Set(value.clone());
        }if self.joined_date.is_some() {
            active_model.joined_date = Set(self.joined_date.clone());
        }if self.last_active.is_some() {
            active_model.last_active = Set(self.last_active.clone());
        }if let Some(value) = &self.password_hash {
            active_model.password_hash = Set(value.clone());
        }if let Some(value) = &self.username {
            active_model.username = Set(value.clone());
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
        .with(crate::models::reading_list::Entity)
        .with(crate::models::review::Entity)
        .all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    let responses: Vec<User> = models.into_iter().map(Into::into).collect();

    Ok(Json(responses))
}

pub async fn create(state: State<AppState>, Json(create): Json<UserCreate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let active_model:ActiveModel = create.into();
    let model = active_model.insert(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to insert item": e.to_string()}))))?;
        let resp: User = model.into();
        Ok(Json(resp))

}

pub async fn patch_one(state: State<AppState>, Path(id): Path<i32>, Json(patch): Json<UserPatch> ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let model = load_item(&state.db, id).await?;
    let mut active_model = model.into_active_model();
    patch.patch_active_model(&mut active_model);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: User = model.into();
    Ok(Json(resp))
}

pub async fn put_one(state: State<AppState>, Path(id): Path<i32>, Json(update): Json<UserUpdate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = load_item(&state.db, id).await?;
    let active_model = update.into_active_model(id);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: User = model.into();
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
        .with(crate::models::reading_list::Entity)
        .with(crate::models::review::Entity)
        .one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "not found"}))))?;
    let resp: User = model.into();
    Ok(Json(resp))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(list))
        .route("/users", post(create))
        .route("/users/{id}", get(read_one))
        .route("/users/{id}", delete(remove))
        .route("/users/{id}", patch(patch_one))
        .route("/users/{id}", put(put_one))
}