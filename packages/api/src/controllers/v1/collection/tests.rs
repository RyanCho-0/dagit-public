use super::*;
use crate::dagit_tests::{TestContext, setup};
use models::v1::collection::{CollectionQuery};

#[tokio::test]
async fn test_create_collection() {
    let TestContext { endpoint, agit_id, .. } = setup().await.unwrap();

    let client = Collection::get_client(&endpoint);
    let result = client.create(agit_id, "New collection Title".to_string()).await;

    assert!(result.is_ok(), "Failed to create collection: {:?}", result.err());
}

#[tokio::test]
async fn test_query_collection() {
    let TestContext { endpoint, agit_id, .. } = setup().await.unwrap();

    let client = Collection::get_client(&endpoint);

    let param = CollectionQuery {
        bookmark: None,
        size: 10,
    };
    let result = client.query(agit_id, param).await;

    assert!(result.is_ok(), "Failed to query collection: {:?}", result.err());
}

#[tokio::test]
async fn test_delete_collection() {
    let TestContext { endpoint, agit_id, .. } = setup().await.unwrap();

    let client = Collection::get_client(&endpoint);

    let result = client.delete(agit_id, 1).await;

    assert!(result.is_ok(), "Failed to delete collection: {:?}", result.err());
}

#[tokio::test]
async fn test_update_collection() {
    let TestContext { endpoint, agit_id, .. } = setup().await.unwrap();

    let client = Collection::get_client(&endpoint);

    let result = client.update(agit_id, 1, "Updated collection Title".to_string()).await;

    assert!(result.is_ok(), "Failed to update collection: {:?}", result.err());
}