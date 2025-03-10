#![allow(unused)]

use by_axum::auth::Authorization;
use by_axum::axum::extract::{Path, Query, State};
use by_axum::axum::routing::post;
use by_axum::axum::{Extension, Json};
use by_axum::{aide, auth};
use by_types::JsonWithHeaders;
use models::Result;
use models::v1::prelude::*;

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
    ) -> Result<Json<User>> {
        tracing::debug!("act_user: {:?}", body);
        match body {
            UserAction::Signup(req) => Ok(Json(ctrl.signup(req, auth).await?)),
            UserAction::UpdateProfile(req) => Ok(Json(ctrl.update_profile(req, auth).await?)),
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
        let user = self
            .repo
            .find_one(&UserReadAction::new().get_user_by_address(principal))
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
