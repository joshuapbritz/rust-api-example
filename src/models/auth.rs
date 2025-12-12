use crate::{
    errors::ServiceError,
    schema::users::{self, dsl},
    utils::crypto::pwdhash,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDTO {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub username: String,
    pub password_hash: String,
    // pub password_salt: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub name: String,
    pub surname: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<UserDTO> for UserResponse {
    fn from(user: UserDTO) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            username: user.username,
            surname: user.surname,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct UserResponseWithToken {
    pub user: UserResponse,
    pub token: String,
}

impl UserResponseWithToken {
    pub fn create(user_dto: UserDTO, token: String) -> Self {
        let user = UserResponse::from(user_dto);
        Self { user, token }
    }
}

#[derive(Deserialize, Serialize)]
pub struct UserRequest {
    pub email: String,
    pub username: String,
    pub name: String,
    pub surname: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsertableUser {
    pub email: String,
    pub username: String,
    pub name: String,
    pub surname: String,
    pub password_hash: String,
    // pub password_salt: String,
}

impl InsertableUser {
    fn validate_email(email: &str) -> Result<String, ServiceError> {
        if EmailAddress::is_valid(email) {
            Ok(String::from(email))
        } else {
            Err(ServiceError::BadRequest)
        }
    }

    pub fn from(user: &UserRequest) -> Result<Self, ServiceError> {
        let hashed = pwdhash::hash_password(&user.password)?;
        let validate_email = Self::validate_email(&user.email)?;

        Ok(Self {
            email: validate_email,
            name: user.name.clone(),
            username: user.username.clone(),
            surname: user.surname.clone(),
            password_hash: hashed,
            // password_salt: hashed.salt,
        })
    }

    pub fn insert(self, db_conn: &mut PgConnection) -> Result<UserDTO, ServiceError> {
        let todo: UserDTO = diesel::insert_into(dsl::users)
            .values(&self)
            .returning(UserDTO::as_returning())
            .get_result(db_conn)?;

        Ok(todo)
    }
}
