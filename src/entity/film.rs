use sea_orm::entity::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use rocket::form::FromForm;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "film")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[field(default = 0)]
    pub tmdb_id: i64,
    pub title: String,
    pub poster_path: String,
    pub year: String,
    pub desc: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::event::Entity")]
    Event,
}

impl Related<super::event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Event.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
