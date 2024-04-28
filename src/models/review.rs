use sea_orm::entity::prelude::*;



#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "review")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub content: String
    ,
    pub helpful_count: Option<i8>
    ,
    #[sea_orm(unique)]
pub novel_id: i32,
#[sea_orm(belongs_to, from = "novel_id", to = "id")]
    pub novel: HasOne<super::novel::Entity>
    ,
    pub rating: String
    ,
    pub spoiler: Option<bool>
    ,
    pub title: Option<String>
    ,
    #[sea_orm(unique)]
pub user_id: i32,
#[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>
    
}

impl ActiveModelBehavior for ActiveModel {}