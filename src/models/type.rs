use sea_orm::entity::prelude::*;



#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "type")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub last_updated: DateTime,
    pub description: Option<String>
    ,
    #[sea_orm(unique)]
    pub name: String
    ,
    #[sea_orm(has_many)]
    pub novels: HasMany<super::novel::Entity>
    
}

impl ActiveModelBehavior for ActiveModel {}