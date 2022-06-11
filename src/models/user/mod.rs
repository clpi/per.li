use std::task::ready;
use std::{
    pin::Pin, 
    alloc::Global};
use api::app::App;
use axum_sessions_auth::{Authentication, HasPermission, AuthSession};
use async_session::{Error as ASError, CookieStore};
use axum_database_sessions::{AxumSession, AxumSessionID, AxumSessionData, AxumDatabasePool};
use futures::{Future, FutureExt};
use time::Instant;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::future::{Future, IntoFuture};

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: Vec<u8>,
    pub email: String,
    pub created: time::Instant,
    pub updated: time::Instant
}
impl Default for User {
    fn default() -> Self {
        Self { 
            created: Instant::now() , 
            updated: Instant::now(),
            ..Default::default() }
    }
}
unsafe impl Send for User {}

#[async_trait::async_trait]
impl Authentication<User> for User {
    async fn is_active(&self) -> bool {
        true 
    }
    fn is_authenticated(&self) -> bool {
        false
    }
    fn is_anonymous(&self) -> bool {
        false 
    }

    // Pin<Box<dyn Future<Output = Result<User,ASError> > + Send+ 'static> >
    // {
    //     async_session::Result::Ok(User::default())
    // }

}
#[async_trait::async_trait]
impl HasPermission for User {
    async fn has(
        &self, 
        perm: &str, 
        pool: &Option<&AxumDatabasePool>
    ) -> bool {
        return match &perm[..] {
            "Token::UseAdmin" => true,
            "Token::ModifyUser" => true,
            _ => false,
        };
    }
}
