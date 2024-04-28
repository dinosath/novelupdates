use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, Clone, Debug, Default, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)", enum_name = "status_origin")]
pub enum StatusOrigin{
        #[default]
        #[sea_orm(string_value = "ongoing")]
        Ongoing,
        #[sea_orm(string_value = "completed")]
        Completed,
        #[sea_orm(string_value = "hiatus")]
        Hiatus,
        #[sea_orm(string_value = "cancelled")]
        Cancelled
}
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "novel")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub alternative_names: Option<String>
    ,
    #[sea_orm(has_many, via = "artist_novel" )]
    pub artists: HasMany<super::artist::Entity>
    ,
    #[sea_orm(has_many, via = "author_novel" )]
    pub authors: HasMany<super::author::Entity>
    ,
    pub average_rating: Option<String>
    ,
    #[sea_orm(has_many, via = "chapter_novel" )]
    pub chapters: HasMany<super::chapter::Entity>
    ,
    pub completely_translated: Option<bool>
    ,
    pub country_of_origin: Option<String>
    ,
    pub cover_image_url: Option<String>
    ,
    pub default_name: String
    ,
    pub description: Option<String>
    ,
    pub english_publisher: Option<String>
    ,
    pub genres: Option<String>
    ,
    pub licensed: Option<bool>
    ,
    pub native_name: Option<String>
    ,
    pub original_language: String
    ,
    #[sea_orm(has_many, via = "novel_publisher" )]
    pub publishers: HasMany<super::publisher::Entity>
    ,
    pub rating_count: Option<i8>
    ,
    #[sea_orm(has_many, via = "novel_reading_list" )]
    pub reading_lists: HasMany<super::reading_list::Entity>
    ,
    pub release_frequency: Option<String>
    ,
    #[sea_orm(has_many)]
    pub reviews: HasMany<super::review::Entity>
    ,
    #[sea_orm(has_many)]
    pub sources: HasMany<super::source::Entity>
    ,
    pub status_origin: Option<StatusOrigin>
    ,
    #[sea_orm(has_many, via = "novel_tag" )]
    pub tags: HasMany<super::tag::Entity>
    ,
    pub total_chapters: Option<i8>
    ,
    #[sea_orm(unique)]
pub type_id: i32,
#[sea_orm(belongs_to, from = "type_id", to = "id")]
    pub r#type: HasOne<super::r#type::Entity>
    ,
    pub views: Option<i8>
    ,
    pub year: Option<i8>
    
}

impl ActiveModelBehavior for ActiveModel {}