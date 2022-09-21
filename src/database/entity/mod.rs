//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use crate::database::sonyflake::Sonyflake;

pub mod prelude;

pub mod applications;
pub mod auth_methods;
pub mod files;
pub mod registration_keys;
pub mod sea_orm_active_enums;
pub mod settings;
pub mod users;
pub mod verifications;

lazy_static! {
    pub static ref DB_SONYFLAKE: Sonyflake =
        Sonyflake::new(0, None).expect("There was a problem creating the Sonyflake worker");
}
