use sea_orm::entity::prelude::*;



#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "publisher")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub country: Option<String>
    ,
    pub description: Option<String>
    ,
    pub name: String
    ,
    #[sea_orm(has_many, via = "novel_publisher" )]
    pub novels: HasMany<super::novel::Entity>
    ,
    pub website: Option<String>
    
}

impl ActiveModelBehavior for ActiveModel {}