#[cfg(test)]
mod tests;

use std::collections::HashMap;

use by_axum::auth::{Authorization, generate_jwt};
use by_axum::axum::extract::{Query, State};
use by_axum::axum::routing::post;
use by_axum::axum::{Extension, Json};
use by_types::{Claims, JsonWithHeaders};
use models::Result;
use models::error::ServiceError;
use models::v1::prelude::*;
use sqlx::postgres::PgRow;

#[derive(Clone, Debug)]
pub struct UserController {
    pool: sqlx::PgPool,
    repo: UserRepository,
}

impl UserController {
    pub fn new(pool: sqlx::PgPool) -> Self {
        let repo = User::get_repository(pool.clone());
        Self { repo, pool }
    }
    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act).get(Self::get))
            .with_state(self.clone()))
    }
}

impl UserController {
    pub async fn act(
        State(ctrl): State<UserController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<UserAction>,
    ) -> Result<JsonWithHeaders<User>> {
        tracing::debug!("act_user: {:?}", auth);
        match body {
            UserAction::Signup(req) => {
                let user = ctrl.signup(req, auth).await?;
                let token = ctrl.generate_token(&user)?;
                Ok(JsonWithHeaders::new(user)
                    .with_bearer_token(&token)
                    .with_cookie(&token))
            }
            UserAction::Login(_) => {
                let user = ctrl.login(auth).await?;
                let token = ctrl.generate_token(&user)?;
                Ok(JsonWithHeaders::new(user)
                    .with_bearer_token(&token)
                    .with_cookie(&token))
            }
            UserAction::UpdateProfile(req) => {
                Ok(JsonWithHeaders::new(ctrl.update_profile(req, auth).await?))
            }
        }
    }

    pub async fn get(
        State(ctrl): State<UserController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(body): Query<UserReadAction>,
    ) -> Result<Json<User>> {
        tracing::debug!("act {:?}", body);
        match body.action {
            Some(UserReadActionType::GetUserByAddress) => Ok(Json(ctrl.login(auth).await?)),
            _ => {
                return Err(models::error::ServiceError::Unknown(
                    "Invalid action".to_string(),
                ));
            }
        }
    }
}

impl UserController {
    fn generate_token(&self, user: &User) -> Result<String> {
        let mut claims = Claims {
            sub: user.address.clone(),
            exp: 0,
            role: by_types::Role::User,
            custom: HashMap::from([
                ("email".to_string(), user.email.to_string()),
                ("id".to_string(), user.id.to_string()),
            ]),
        };
        Ok(generate_jwt(&mut claims).map_err(|e| {
            tracing::error!("jwt generation error: {:?}", e);
            ServiceError::JwtGenerationFailed(e.to_string())
        })?)
    }
    fn get_principal(&self, auth: Option<Authorization>) -> Result<String> {
        match auth {
            Some(Authorization::UserSig(sig)) => {
                let principal = sig
                    .principal()
                    .map_err(|_| models::error::ServiceError::Unauthorized)?;
                Ok(principal)
            }
            _ => Err(models::error::ServiceError::Unauthorized),
        }
    }

    pub async fn login(&self, auth: Option<Authorization>) -> Result<User> {
        let principal = self.get_principal(auth)?;
        let user = User::query_builder()
            .address_equals(principal)
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }
    pub async fn signup(
        &self,
        req: UserSignupRequest,
        auth: Option<Authorization>,
    ) -> Result<User> {
        let principal = self.get_principal(auth)?;

        let user = self
            .repo
            .insert(
                req.provider,
                principal,
                req.email,
                req.name,
                req.profile_url,
            )
            .await?;
        Ok(user)
    }

    pub async fn update_profile(
        &self,
        req: UserUpdateProfileRequest,
        auth: Option<Authorization>,
    ) -> Result<User> {
        let principal = self.get_principal(auth)?;
        let user = self
            .repo
            .find_one(&UserReadAction::new().get_user_by_address(principal))
            .await?;
        let user = self
            .repo
            .update(
                user.id,
                UserRepositoryUpdateRequest::new().with_name(req.name),
            )
            .await?;
        Ok(user)
    }
}
