use serde_json::json;
use axum::{Router, extract::{Path, State}, http::StatusCode, routing::{delete, get, patch, post, put}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ModelTrait, EntityTrait, Set, IntoActiveModel, ConnectionTrait};
use sea_orm::EntityLoaderTrait;
use sea_orm::prelude::*;
use crate::app_state::AppState;
use crate::models::novel::{ActiveModel, Entity, Model, ModelEx, StatusOrigin};
use super::{artist::Artist as Artist, author::Author as Author, chapter::Chapter as Chapter, publisher::Publisher as Publisher, reading_list::ReadingList as ReadingList, review::Review as Review, source::Source as Source, tag::Tag as Tag, r#type::Type as Type, };
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Novel {
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub alternative_names: Option<String>,
    pub artists: Option<Vec<Artist>>,
    pub authors: Option<Vec<Author>>,
    pub average_rating: Option<String>,
    pub chapters: Option<Vec<Chapter>>,
    pub completely_translated: Option<bool>,
    pub country_of_origin: Option<String>,
    pub cover_image_url: Option<String>,
    pub default_name: String,
    pub description: Option<String>,
    pub english_publisher: Option<String>,
    pub genres: Option<String>,
    pub licensed: Option<bool>,
    pub native_name: Option<String>,
    pub original_language: String,
    pub publishers: Option<Vec<Publisher>>,
    pub rating_count: Option<i8>,
    pub reading_lists: Option<Vec<ReadingList>>,
    pub release_frequency: Option<String>,
    pub reviews: Option<Vec<Review>>,
    pub sources: Option<Vec<Source>>,
    pub status_origin: Option<StatusOrigin>,
    pub tags: Option<Vec<Tag>>,
    pub total_chapters: Option<i8>,
    pub r#type: Type,
    pub views: Option<i8>,
    pub year: Option<i8>
    
}

impl From<Model> for Novel {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            alternative_names: model.alternative_names,
            artists: None,
            authors: None,
            average_rating: model.average_rating,
            chapters: None,
            completely_translated: model.completely_translated,
            country_of_origin: model.country_of_origin,
            cover_image_url: model.cover_image_url,
            default_name: model.default_name,
            description: model.description,
            english_publisher: model.english_publisher,
            genres: model.genres,
            licensed: model.licensed,
            native_name: model.native_name,
            original_language: model.original_language,
            publishers: None,
            rating_count: model.rating_count,
            reading_lists: None,
            release_frequency: model.release_frequency,
            reviews: vec![].into(),
            sources: vec![].into(),
            status_origin: model.status_origin,
            tags: None,
            total_chapters: model.total_chapters,
            r#type: Type::default(),
            views: model.views,
            year: model.year
            
        }
    }
}

impl From<ModelEx> for Novel {
    fn from(model: ModelEx) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            last_updated: model.last_updated,
            alternative_names: model.alternative_names,
            artists: Some(model.artists.into_iter().map(Artist::from).collect()),
            authors: Some(model.authors.into_iter().map(Author::from).collect()),
            average_rating: model.average_rating,
            chapters: Some(model.chapters.into_iter().map(Chapter::from).collect()),
            completely_translated: model.completely_translated,
            country_of_origin: model.country_of_origin,
            cover_image_url: model.cover_image_url,
            default_name: model.default_name,
            description: model.description,
            english_publisher: model.english_publisher,
            genres: model.genres,
            licensed: model.licensed,
            native_name: model.native_name,
            original_language: model.original_language,
            publishers: Some(model.publishers.into_iter().map(Publisher::from).collect()),
            rating_count: model.rating_count,
            reading_lists: Some(model.reading_lists.into_iter().map(ReadingList::from).collect()),
            release_frequency: model.release_frequency,
            reviews: Some(model.reviews.into_iter().map(Review::from).collect()),
            sources: Some(model.sources.into_iter().map(Source::from).collect()),
            status_origin: model.status_origin,
            tags: Some(model.tags.into_iter().map(Tag::from).collect()),
            total_chapters: model.total_chapters,
            r#type: model.r#type.into_option().map(Into::into).unwrap_or_default(),
            views: model.views,
            year: model.year,
            
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NovelCreate {
    pub alternative_names: Option<String>,
    pub artists: Option<Vec<Artist>>,
    pub authors: Option<Vec<Author>>,
    pub average_rating: Option<String>,
    pub chapters: Option<Vec<Chapter>>,
    pub completely_translated: Option<bool>,
    pub country_of_origin: Option<String>,
    pub cover_image_url: Option<String>,
    pub default_name: String,
    pub description: Option<String>,
    pub english_publisher: Option<String>,
    pub genres: Option<String>,
    pub licensed: Option<bool>,
    pub native_name: Option<String>,
    pub original_language: String,
    pub publishers: Option<Vec<Publisher>>,
    pub rating_count: Option<i8>,
    pub reading_lists: Option<Vec<ReadingList>>,
    pub release_frequency: Option<String>,
    pub reviews: Option<Vec<Review>>,
    pub sources: Option<Vec<Source>>,
    pub status_origin: Option<StatusOrigin>,
    pub tags: Option<Vec<Tag>>,
    pub total_chapters: Option<i8>,
    pub r#type: Type,
    pub views: Option<i8>,
    pub year: Option<i8>
    
}

impl From<NovelCreate> for ActiveModel {
    fn from(source: NovelCreate) -> Self {
        ActiveModel {
            alternative_names: Set(source.alternative_names.clone()),
            average_rating: Set(source.average_rating.clone()),
            completely_translated: Set(source.completely_translated.clone()),
            country_of_origin: Set(source.country_of_origin.clone()),
            cover_image_url: Set(source.cover_image_url.clone()),
            default_name: Set(source.default_name.clone()),
            description: Set(source.description.clone()),
            english_publisher: Set(source.english_publisher.clone()),
            genres: Set(source.genres.clone()),
            licensed: Set(source.licensed.clone()),
            native_name: Set(source.native_name.clone()),
            original_language: Set(source.original_language.clone()),
            rating_count: Set(source.rating_count.clone()),
            release_frequency: Set(source.release_frequency.clone()),
            status_origin: Set(source.status_origin.clone()),
            total_chapters: Set(source.total_chapters.clone()),
            type_id: Set(source.r#type.id.clone()),views: Set(source.views.clone()),
            year: Set(source.year.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NovelUpdate {
    pub alternative_names: Option<String>,
    pub artists: Option<Vec<Artist>>,
    pub authors: Option<Vec<Author>>,
    pub average_rating: Option<String>,
    pub chapters: Option<Vec<Chapter>>,
    pub completely_translated: Option<bool>,
    pub country_of_origin: Option<String>,
    pub cover_image_url: Option<String>,
    pub default_name: String,
    pub description: Option<String>,
    pub english_publisher: Option<String>,
    pub genres: Option<String>,
    pub licensed: Option<bool>,
    pub native_name: Option<String>,
    pub original_language: String,
    pub publishers: Option<Vec<Publisher>>,
    pub rating_count: Option<i8>,
    pub reading_lists: Option<Vec<ReadingList>>,
    pub release_frequency: Option<String>,
    pub reviews: Option<Vec<Review>>,
    pub sources: Option<Vec<Source>>,
    pub status_origin: Option<StatusOrigin>,
    pub tags: Option<Vec<Tag>>,
    pub total_chapters: Option<i8>,
    pub r#type: Type,
    pub views: Option<i8>,
    pub year: Option<i8>
    
}

impl NovelUpdate {
    fn into_active_model(self, id:i32) -> ActiveModel {
        ActiveModel {
            id: Set(id),
            alternative_names: Set(self.alternative_names.clone()),
            average_rating: Set(self.average_rating.clone()),
            completely_translated: Set(self.completely_translated.clone()),
            country_of_origin: Set(self.country_of_origin.clone()),
            cover_image_url: Set(self.cover_image_url.clone()),
            default_name: Set(self.default_name.clone()),
            description: Set(self.description.clone()),
            english_publisher: Set(self.english_publisher.clone()),
            genres: Set(self.genres.clone()),
            licensed: Set(self.licensed.clone()),
            native_name: Set(self.native_name.clone()),
            original_language: Set(self.original_language.clone()),
            rating_count: Set(self.rating_count.clone()),
            release_frequency: Set(self.release_frequency.clone()),
            status_origin: Set(self.status_origin.clone()),
            total_chapters: Set(self.total_chapters.clone()),
            type_id: Set(self.r#type.id.clone()),
            views: Set(self.views.clone()),
            year: Set(self.year.clone()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NovelPatch {
    pub alternative_names: Option<String>,
    pub artists: Option<Vec<Artist>>,
    pub authors: Option<Vec<Author>>,
    pub average_rating: Option<String>,
    pub chapters: Option<Vec<Chapter>>,
    pub completely_translated: Option<bool>,
    pub country_of_origin: Option<String>,
    pub cover_image_url: Option<String>,
    pub default_name: Option<String>,
    pub description: Option<String>,
    pub english_publisher: Option<String>,
    pub genres: Option<String>,
    pub licensed: Option<bool>,
    pub native_name: Option<String>,
    pub original_language: Option<String>,
    pub publishers: Option<Vec<Publisher>>,
    pub rating_count: Option<i8>,
    pub reading_lists: Option<Vec<ReadingList>>,
    pub release_frequency: Option<String>,
    pub reviews: Option<Vec<Review>>,
    pub sources: Option<Vec<Source>>,
    pub status_origin: Option<StatusOrigin>,
    pub tags: Option<Vec<Tag>>,
    pub total_chapters: Option<i8>,
    pub r#type: Option<Type>,
    pub views: Option<i8>,
    pub year: Option<i8>
    
}

impl NovelPatch {
    pub fn patch_active_model(&self, active_model: &mut ActiveModel) {
        if self.alternative_names.is_some() {
            active_model.alternative_names = Set(self.alternative_names.clone());
        }if self.average_rating.is_some() {
            active_model.average_rating = Set(self.average_rating.clone());
        }if self.completely_translated.is_some() {
            active_model.completely_translated = Set(self.completely_translated.clone());
        }if self.country_of_origin.is_some() {
            active_model.country_of_origin = Set(self.country_of_origin.clone());
        }if self.cover_image_url.is_some() {
            active_model.cover_image_url = Set(self.cover_image_url.clone());
        }if let Some(value) = &self.default_name {
            active_model.default_name = Set(value.clone());
        }if self.description.is_some() {
            active_model.description = Set(self.description.clone());
        }if self.english_publisher.is_some() {
            active_model.english_publisher = Set(self.english_publisher.clone());
        }if self.genres.is_some() {
            active_model.genres = Set(self.genres.clone());
        }if self.licensed.is_some() {
            active_model.licensed = Set(self.licensed.clone());
        }if self.native_name.is_some() {
            active_model.native_name = Set(self.native_name.clone());
        }if let Some(value) = &self.original_language {
            active_model.original_language = Set(value.clone());
        }if self.rating_count.is_some() {
            active_model.rating_count = Set(self.rating_count.clone());
        }if self.release_frequency.is_some() {
            active_model.release_frequency = Set(self.release_frequency.clone());
        }if self.status_origin.is_some() {
            active_model.status_origin = Set(self.status_origin.clone());
        }if self.total_chapters.is_some() {
            active_model.total_chapters = Set(self.total_chapters.clone());
        }if let Some(value) = &self.r#type {
            active_model.type_id = Set(value.id.clone());
        }if self.views.is_some() {
            active_model.views = Set(self.views.clone());
        }if self.year.is_some() {
            active_model.year = Set(self.year.clone());
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
        .with(crate::models::artist::Entity)
        .with(crate::models::author::Entity)
        .with(crate::models::chapter::Entity)
        .with(crate::models::publisher::Entity)
        .with(crate::models::reading_list::Entity)
        .with(crate::models::review::Entity)
        .with(crate::models::source::Entity)
        .with(crate::models::tag::Entity)
        .with(crate::models::r#type::Entity)
        .all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    let responses: Vec<Novel> = models.into_iter().map(Into::into).collect();

    Ok(Json(responses))
}

pub async fn create(state: State<AppState>, Json(create): Json<NovelCreate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let active_model:ActiveModel = create.into();
    let model = active_model.insert(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to insert item": e.to_string()}))))?;
        let resp: Novel = model.into();
        Ok(Json(resp))

}

pub async fn patch_one(state: State<AppState>, Path(id): Path<i32>, Json(patch): Json<NovelPatch> ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let model = load_item(&state.db, id).await?;
    let mut active_model = model.into_active_model();
    patch.patch_active_model(&mut active_model);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: Novel = model.into();
    Ok(Json(resp))
}

pub async fn put_one(state: State<AppState>, Path(id): Path<i32>, Json(update): Json<NovelUpdate>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = load_item(&state.db, id).await?;
    let active_model = update.into_active_model(id);
    let model = active_model.update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"failed to update item": e.to_string()}))))?;
    let resp: Novel = model.into();
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
        .with(crate::models::artist::Entity)
        .with(crate::models::author::Entity)
        .with(crate::models::chapter::Entity)
        .with(crate::models::publisher::Entity)
        .with(crate::models::reading_list::Entity)
        .with(crate::models::review::Entity)
        .with(crate::models::source::Entity)
        .with(crate::models::tag::Entity)
        .with(crate::models::r#type::Entity)
        .one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "not found"}))))?;
    let resp: Novel = model.into();
    Ok(Json(resp))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/novels", get(list))
        .route("/novels", post(create))
        .route("/novels/{id}", get(read_one))
        .route("/novels/{id}", delete(remove))
        .route("/novels/{id}", patch(patch_one))
        .route("/novels/{id}", put(put_one))
}