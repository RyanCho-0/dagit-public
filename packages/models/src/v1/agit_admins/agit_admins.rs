#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/", table = agit_admins)]
pub struct AgitAdmins {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = agits)]
    pub agit_id: i64,
    #[api_model(summary, many_to_one = users)]
    pub user_id: i64,
}
