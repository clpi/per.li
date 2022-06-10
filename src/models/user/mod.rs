use std::{pin::Pin, alloc::Global};
use axum::{
    async_trait,
};
use axum_sessions_auth::{Authentication, HasPermission};
use axum_database_sessions::{AxumSession, AxumSessionID, AxumSessionData, AxumDatabasePool};
use futures::Future;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: Vec<u8>,
    pub email: String,
    pub created: time::Instant
    
}

#[async_trait]
impl HasPermission for User {
    async fn has(
        &self, 
        perm: &str, 
        pool: &Option<&AxumDatabasePool>
    ) -> bool {
        match &perm[..] {
            "Token::UseAdmin" => true,
            "Token::ModifyUser" => true,
            _ => false,
        }
    }
}
