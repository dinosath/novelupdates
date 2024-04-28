use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, Clone, Debug, Default, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)", enum_name = "status")]
pub enum Status{
        #[default]
        #[sea_orm(string_value = "reading")]
        Reading,
        #[sea_orm(string_value = "completed")]
        Completed,
        #[sea_orm(string_value = "plan_to_read")]
        PlanToRead,
        #[sea_orm(string_value = "on_hold")]
        OnHold,
        #[sea_orm(string_value = "dropped")]
        Dropped
}
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "reading_list")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub completed_date: Option<String>
    ,
    pub current_chapter: Option<i8>
    ,
    pub last_read: Option<String>
    ,
    pub notes: Option<String>
    ,
    #[sea_orm(has_many, via = "novel_reading_list" )]
    pub novel: HasMany<super::novel::Entity>
    ,
    pub personal_rating: Option<String>
    ,
    pub started_date: Option<String>
    ,
    pub status: Status
    ,
    #[sea_orm(unique)]
pub user_id: i32,
#[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>
    
}

impl ActiveModelBehavior for ActiveModel {}