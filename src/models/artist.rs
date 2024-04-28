use sea_orm::entity::prelude::*;



#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "artist")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub bio: Option<String>
    ,
    pub name: String
    ,
    pub native_name: Option<String>
    ,
    #[sea_orm(has_many, via = "artist_novel" )]
    pub novels: HasMany<super::novel::Entity>
    ,
    pub website: Option<String>
    
}

impl ActiveModelBehavior for ActiveModel {}