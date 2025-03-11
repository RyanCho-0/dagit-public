use super::*;
use crate::dagit_tests::{TestContext, setup};
use models::v1::agit::{AgitQuery};

#[tokio::test]
async fn test_create_agit() {
    let TestContext { endpoint, .. } = setup().await.unwrap();

    let client = Agit::get_client(&endpoint);
    let result = client.create("New Agit Title".to_string()).await;

    assert!(result.is_ok(), "Failed to create agit: {:?}", result.err());
}

#[tokio::test]
async fn test_query_agit() {
    let TestContext { endpoint, .. } = setup().await.unwrap();

    let client = Agit::get_client(&endpoint);

    let param = AgitQuery {
        bookmark: None,
        size: 10,
    };
    let result = client.query(param).await;

    assert!(result.is_ok(), "Failed to query agit: {:?}", result.err());
}

#[tokio::test]
async fn test_delete_agit() {
    let TestContext { endpoint, .. } = setup().await.unwrap();

    let client = Agit::get_client(&endpoint);

    let result = client.delete(1).await;

    assert!(result.is_ok(), "Failed to delete agit: {:?}", result.err());
}

#[tokio::test]
async fn test_update_agit() {
    let TestContext { endpoint, .. } = setup().await.unwrap();

    let client = Agit::get_client(&endpoint);

    let result = client.update(1, "Updated Agit Title".to_string()).await;

    assert!(result.is_ok(), "Failed to update agit: {:?}", result.err());
}