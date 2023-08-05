//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use super::sea_orm_active_enums::AccountStatus;
use super::sea_orm_active_enums::Gender;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_information")]
pub struct Model {
    #[sea_orm(unique)]
    pub id: Uuid,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub middlename: Option<String>,
    #[sea_orm(unique)]
    pub username: Option<String>,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub account_status: Option<AccountStatus>,
    pub date_of_birth: Option<Date>,
    pub gender: Option<Gender>,
    pub avatar: Option<String>,
    pub phone_number: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub last_available_at: Option<DateTime>,
    pub fullname: Option<String>,
    pub otp_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::one_time_passwords::Entity",
        from = "Column::OtpId",
        to = "super::one_time_passwords::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    OneTimePasswords,
}

impl Related<super::one_time_passwords::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OneTimePasswords.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
