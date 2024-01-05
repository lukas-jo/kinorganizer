use sea_orm::entity::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use rocket::form::FromForm;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "event")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[field(default = 0)]
    pub id: i32,
    pub film: i64,
    pub text: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
