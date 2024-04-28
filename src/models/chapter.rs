use sea_orm::entity::prelude::*;



#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "chapter")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub content_url: Option<String>
    ,
    pub is_locked: Option<bool>
    ,
    pub language: String
    ,
    #[sea_orm(has_many, via = "chapter_novel" )]
    pub novels: HasMany<super::novel::Entity>
    ,
    pub number: String
    ,
    pub part: Option<i8>
    ,
    pub release_date: Option<String>
    ,
    #[sea_orm(unique)]
pub source_id: i32,
#[sea_orm(belongs_to, from = "source_id", to = "id")]
    pub source: HasOne<super::source::Entity>
    ,
    pub title: String
    ,
    pub views: Option<i8>
    ,
    pub volume: Option<i8>
    
}

impl ActiveModelBehavior for ActiveModel {}