#![allow(unused)]
use by_macros::{ApiModel, api_model};

#[cfg(feature = "server")]
use by_axum::aide;

#[derive(validator::Validate)]
#[api_model(base = "/v1/users", table = users, action = [login])]
pub struct User {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(type = TIMESTAMP, auto = insert)]
    pub created_at: i64,
    #[api_model(type = TIMESTAMP, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(unique, action = signup, type = INTEGER)]
    pub provider: AuthProvider,
    #[api_model(unique, read_action = get_user_by_address)]
    pub address: String,

    #[api_model(action = signup)]
    #[validate(email)]
    pub email: String,
    #[api_model(action = [signup, update_profile])]
    pub name: String,

    #[api_model(action = [signup, update_profile], nullable)]
    #[validate(url)]
    pub profile_url: Option<String>,

    #[api_model(one_to_many = credits, foreign_key = user_id, aggregator = sum(credit))]
    pub credits: i64,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, ApiModel)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum AuthProvider {
    #[default]
    Google = 1,
    Icp = 2,
}
