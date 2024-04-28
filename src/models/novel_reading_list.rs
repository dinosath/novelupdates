use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "novel_reading_list")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub novel_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub reading_list_id: i32,
    #[sea_orm(belongs_to, from = "novel_id", to = "id")]
    pub novel: Option<super::novel::Entity>,
    #[sea_orm(belongs_to, from = "reading_list_id", to = "id")]
    pub reading_list: Option<super::reading_list::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}