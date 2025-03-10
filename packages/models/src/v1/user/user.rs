#![allow(unused)]
use by_macros::{ApiModel, api_model};

#[cfg(feature = "server")]
use by_axum::aide;

#[derive(validator::Validate)]
#[api_model(base = "/v1/users", table = users)]
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

    // #[api_model(action = signup)]
    // pub password: String,
    #[api_model(action = [signup, update_profile], nullable)]
    #[validate(url)]
    pub profile_url: Option<String>,

    #[api_model(one_to_many = credits, foreign_key = user_id, aggregator = sum(credit))]
    pub credits: i64,
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserResponse {
    #[serde(flatten)]
    pub user: User,
    pub action: UserResponseType,
}

#[derive(Debug, Clone, Eq, PartialEq, ApiModel, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum UserResponseType {
    #[default]
    SignUp = 1,
    Login = 2,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, ApiModel)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum AuthProvider {
    #[default]
    Google = 1,
    Kakao = 2,
    Icp = 3,
}

#[api_model(base = "/v1/users/:user-id/credits", table = credits)]
pub struct UserCredit {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(many_to_one = users)]
    pub user_id: i64,

    #[api_model(summary, action = add)]
    pub description: String,
    #[api_model(summary, action = add)]
    pub credit: i64,
}
