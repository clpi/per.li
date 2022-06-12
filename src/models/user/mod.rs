use crate::app::App;
use async_session::{CookieStore, Error as ASError};
use axum_database_sessions::{AxumDatabasePool, AxumSession, AxumSessionData, AxumSessionID};
use axum_sessions_auth::{AuthSession, Authentication, HasPermission};
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use std::future::{self, ready, Future};
use std::pin::Pin;
use time::{OffsetDateTime, serde::rfc3339};
use uuid::Uuid;
use prost::{self, Enumeration, Message};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub username: String,
    #[serde(default = "User::password_default")]
    pub password: Vec<u8>,
    pub email: String,
    #[serde(with = "rfc3339")]
    pub created: time::OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub updated: time::OffsetDateTime,
}
impl Default for User {
    fn default() -> Self {
        Self {
            created: time::OffsetDateTime::now_utc(),
            updated: time::OffsetDateTime::now_utc(),
            ..Default::default()
        }
    }
}
impl User {
    pub fn password_default() -> Vec<u8> {
        vec![0; 32]
    }
}
unsafe impl Send for User {}

// #[async_trait::async_trait]
// impl Authentication<User> for User {
//     async fn is_active(&self) -> bool {
//         true
//     }
//     fn is_authenticated(&self) -> bool {
//         false
//     }
//     fn is_anonymous(&self) -> bool {
//         false
//     }
//
//     // Pin<Box<dyn Future<Output = Result<User,ASError> > + Send+ 'static> >
//     // {
//     //     async_session::Result::Ok(User::default())
//     // }
//
// }
// #[async_trait::async_trait]
// impl HasPermission for User {
//     async fn has(
//         &self,
//         perm: &str,
//         pool: &Option<&AxumDatabasePool>
//     ) -> bool {
//         return match &perm[..] {
//             "Token::UseAdmin" => true,
//             "Token::ModifyUser" => true,
//             _ => false,
//         };
//     }
// }
