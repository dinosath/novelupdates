use sea_orm::entity::prelude::*;



#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub avatar_url: Option<String>
    ,
    pub bio: Option<String>
    ,
    pub display_name: Option<String>
    ,
    #[sea_orm(unique)]
    pub email: String
    ,
    pub joined_date: Option<String>
    ,
    pub last_active: Option<String>
    ,
    pub password_hash: String
    ,
    #[sea_orm(has_many)]
    pub reading_lists: HasMany<super::reading_list::Entity>
    ,
    #[sea_orm(has_many)]
    pub reviews: HasMany<super::review::Entity>
    ,
    #[sea_orm(unique)]
    pub username: String
    
}

impl ActiveModelBehavior for ActiveModel {}