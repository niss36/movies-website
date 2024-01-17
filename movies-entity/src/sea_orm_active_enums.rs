//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.11

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "credit_type")]
pub enum CreditType {
    #[sea_orm(string_value = "actor")]
    Actor,
    #[sea_orm(string_value = "director")]
    Director,
    #[sea_orm(string_value = "producer")]
    Producer,
}
