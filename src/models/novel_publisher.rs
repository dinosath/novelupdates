use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "novel_publisher")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub novel_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub publisher_id: i32,
    #[sea_orm(belongs_to, from = "novel_id", to = "id")]
    pub novel: Option<super::novel::Entity>,
    #[sea_orm(belongs_to, from = "publisher_id", to = "id")]
    pub publisher: Option<super::publisher::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}