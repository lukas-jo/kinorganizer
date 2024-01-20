use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize)]
#[sea_orm(table_name = "event")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub tmdb_id: i64,
    pub text: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::film::Entity",
        from = "Column::TmdbId",
        to = "super::film::Column::TmdbId"
    )]
    Film,
}

impl Related<super::film::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Film.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
