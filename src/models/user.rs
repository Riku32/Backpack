use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UserData {
    pub id: String,

    #[serde(skip_serializing)]
    pub password: String,

    pub username: String,
    pub email: String,
    pub verified: bool,
    pub role: UserRole,
}

/// User access level
#[derive(Serialize, Deserialize, sqlx::Type, Eq, PartialEq, PartialOrd)]
#[serde(rename_all(serialize = "lowercase", deserialize = "PascalCase"))]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
}

#[derive(Deserialize)]
pub struct UserCreateForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserDeleteForm {
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserEmailForm {
    pub email: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordChangeForm {
    pub current_password: String,
    pub new_password: String,
}
