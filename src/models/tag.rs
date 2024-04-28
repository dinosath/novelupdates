use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, Clone, Debug, Default, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)", enum_name = "category")]
pub enum Category{
        #[default]
        #[sea_orm(string_value = "genre")]
        Genre,
        #[sea_orm(string_value = "theme")]
        Theme,
        #[sea_orm(string_value = "plot")]
        Plot,
        #[sea_orm(string_value = "character")]
        Character,
        #[sea_orm(string_value = "setting")]
        Setting,
        #[sea_orm(string_value = "other")]
        Other
}
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub category: Option<Category>
    ,
    pub description: Option<String>
    ,
    pub name: String
    ,
    #[sea_orm(has_many, via = "novel_tag" )]
    pub novels: HasMany<super::novel::Entity>
    ,
    #[sea_orm(unique)]
    pub slug: String
    ,
    pub usage_count: Option<i8>
    
}

impl ActiveModelBehavior for ActiveModel {}