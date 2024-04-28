use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, Clone, Debug, Default, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)", enum_name = "status")]
pub enum Status{
        #[default]
        #[sea_orm(string_value = "active")]
        Active,
        #[sea_orm(string_value = "inactive")]
        Inactive,
        #[sea_orm(string_value = "completed")]
        Completed,
        #[sea_orm(string_value = "dropped")]
        Dropped
}
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "source")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub chapter_count: Option<i8>
    ,
    #[sea_orm(has_many)]
    pub chapters: HasMany<super::chapter::Entity>
    ,
    pub completely_translated: Option<bool>
    ,
    #[sea_orm(unique)]
pub group_id: i32,
#[sea_orm(belongs_to, from = "group_id", to = "id")]
    pub group: HasOne<super::group::Entity>
    ,
    pub is_official: Option<bool>
    ,
    pub language: String
    ,
    pub name: String
    ,
    #[sea_orm(unique)]
pub novel_id: i32,
#[sea_orm(belongs_to, from = "novel_id", to = "id")]
    pub novel: HasOne<super::novel::Entity>
    ,
    pub status: Option<Status>
    ,
    pub url: Option<String>
    
}

impl ActiveModelBehavior for ActiveModel {}