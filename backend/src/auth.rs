use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use secrecy::{ExposeSecret as _, SecretString};
use serde::{Deserialize, Serialize};

use crate::db;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &[]
    }
}

#[derive(Debug, Clone)]
pub struct Backend {
    db: DatabaseConnection,
}

#[derive(Debug)]
pub enum Error {
    DatabaseError(sea_orm::DbErr),
    Argon2Error(argon2::password_hash::Error),
}

impl From<sea_orm::DbErr> for Error {
    fn from(err: sea_orm::DbErr) -> Self {
        Error::DatabaseError(err)
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(err: argon2::password_hash::Error) -> Self {
        Error::Argon2Error(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DatabaseError(err) => write!(f, "Database Error: {}", err),
            Error::Argon2Error(err) => write!(f, "Argon2 Error: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::DatabaseError(err) => Some(err),
            Error::Argon2Error(err) => Some(err),
        }
    }
}

impl Backend {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Create a user with a password
    pub async fn create_user(&self, username: String, password: SecretString) -> Result<(), Error> {
        // Hash the password
        let password_hash = Argon2::default()
            .hash_password(
                password.expose_secret().as_bytes(),
                &SaltString::generate(&mut OsRng),
            )?
            .to_string();

        // Try to get an existing user
        let user_entity = db::users::Entity::find()
            .filter(db::users::Column::Username.eq(&username))
            .one(&self.db)
            .await?;

        // If the user does not exist then add them
        if user_entity.is_none() {
            db::users::Entity::insert(db::users::ActiveModel {
                username: Set(username),
                password_hash: Set(password_hash),
                ..Default::default()
            })
            .exec(&self.db)
            .await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: SecretString,
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        // Get the user entity in the datbase
        let user_entity = db::users::Entity::find()
            .filter(db::users::Column::Username.eq(credentials.username.clone()))
            .one(&self.db)
            .await?;

        // If the user exists...
        if let Some(entity) = user_entity {
            let password_hash = PasswordHash::new(&entity.password_hash)?;
            if Argon2::default()
                .verify_password(credentials.password.expose_secret().as_bytes(), &password_hash)
                .is_ok()
            {
                Ok(Some(User {
                    id: entity.id,
                    username: entity.username,
                }))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user_entity = db::users::Entity::find_by_id(*user_id).one(&self.db).await?;
        Ok(user_entity.map(|entity| User {
            id: entity.id,
            username: entity.username,
        }))
    }
}
