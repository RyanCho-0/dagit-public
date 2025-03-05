#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

#[api_model(base = "/artworks", table = artworks)]
pub struct Artwork {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = agits)]
    pub agit_id: i64,

    #[api_model(summary, many_to_one = collections, nullable)]
    pub collection_id: Option<i64>,

    #[api_model(summary, many_to_one = artists)]
    pub artist_id: i64,
}
