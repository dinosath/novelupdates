use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "author_novel")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub author_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub novel_id: i32,
    #[sea_orm(belongs_to, from = "author_id", to = "id")]
    pub author: Option<super::author::Entity>,
    #[sea_orm(belongs_to, from = "novel_id", to = "id")]
    pub novel: Option<super::novel::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}