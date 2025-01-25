

use std::sync::Arc;
use async_trait::async_trait;
use tracing::error;

use crate::err::{Error, RepoResult}; // Adjust to match your actual error/Result types
use crate::domain::UserCreationInfo;
use crate::user_service::repository::UserRepository; // The trait
use super::Repository;

#[async_trait]
impl UserRepository for Repository {
    /// Creates a new user in the database using the provided UserCreationInfo.
    async fn create_user(&self, user_creation_info: UserCreationInfo) -> Result<(), Error> {
        // Obtain a libsql connection using your existing helper method
        let conn = self.get_connection().await?;

        // Example user table insert
        // Adjust field names and query based on your actual schema
        if let Err(err) = conn.execute(
            "INSERT INTO users (email, phone_number, created_at) VALUES (?1, ?2, strftime('%s','now'))",
            libsql::params![
                user_creation_info.email,
                user_creation_info.phone_number,
            ],
        )
        .await
        {
            error!("Error creating user <{}>, phone <{}>: {err}", 
                    user_creation_info.email, 
                    user_creation_info.phone_number);
            return Err(Error::InternalDbError(format!("DB insert failed: {err}").into()));
        }

        // Return Ok if successful
        Ok(())
    }

    /// Looks up a user ID by email address.
    /// Returns Ok containing the user ID as a String if found, or an error otherwise.
    async fn get_user_id_by_email(&self, email: &String) -> Result<String, Error> {
        let conn = self.get_connection().await?;
        let mut rows = conn
            .query(
                "SELECT id FROM users WHERE email = ?1 LIMIT 1",
                libsql::params![email],
            )
            .await?;

        if let Some(row_result) = rows.next().await? {
            // Here we assume the ID is stored as an integer
            let user_id: i64 = row_result.get(0)?;
            Ok(user_id.to_string())
        } else {
            // If no row is found, return an error
            Err(Error::InternalDbError(
                "No user found with that email".into(),
            ))
        }
    }

    /// Looks up a user ID by phone number.
    /// Returns Ok containing the user ID as a String if found, or an error otherwise.
    async fn get_user_id_by_phone_number(&self, phone_number: &String) -> Result<String, Error> {
        let conn = self.get_connection().await?;
        let mut rows = conn
            .query(
                "SELECT id FROM users WHERE phone_number = ?1 LIMIT 1",
                libsql::params![phone_number],
            )
            .await?;

        if let Some(row_result) = rows.next().await? {
            // Again, assuming the ID is an integer in the database
            let user_id: i64 = row_result.get(0)?;
            Ok(user_id.to_string())
        } else {
            Err(Error::InternalDbError(
                "No user found with that phone number".into(),
            ))
        }
    }
}
