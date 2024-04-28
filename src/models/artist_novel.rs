use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "artist_novel")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub artist_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub novel_id: i32,
    #[sea_orm(belongs_to, from = "artist_id", to = "id")]
    pub artist: Option<super::artist::Entity>,
    #[sea_orm(belongs_to, from = "novel_id", to = "id")]
    pub novel: Option<super::novel::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}