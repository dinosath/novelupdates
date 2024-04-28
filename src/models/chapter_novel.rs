use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "chapter_novel")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub chapter_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub novel_id: i32,
    #[sea_orm(belongs_to, from = "chapter_id", to = "id")]
    pub chapter: Option<super::chapter::Entity>,
    #[sea_orm(belongs_to, from = "novel_id", to = "id")]
    pub novel: Option<super::novel::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}