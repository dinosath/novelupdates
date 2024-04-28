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
        #[sea_orm(string_value = "disbanded")]
        Disbanded
}
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "group")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub description: Option<String>
    ,
    pub discord_url: Option<String>
    ,
    pub founded_date: Option<String>
    ,
    pub language: Option<String>
    ,
    pub logo_url: Option<String>
    ,
    pub member_count: Option<i8>
    ,
    pub name: String
    ,
    pub patreon_url: Option<String>
    ,
    #[sea_orm(has_many)]
    pub sources: HasMany<super::source::Entity>
    ,
    pub status: Option<Status>
    ,
    pub website: Option<String>
    
}

impl ActiveModelBehavior for ActiveModel {}