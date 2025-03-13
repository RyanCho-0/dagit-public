use super::*;
use crate::dagit_tests::{TestContext, setup};
use models::v1::collections::CollectionQuery;

const IMAGE_URL: &str =
    "https://metadata.dev.dagit.club/images/72a11429-20c0-4d62-8cde-ff3d4d5dc0bb";

#[tokio::test]
async fn test_create_collection() -> Collection {
    let TestContext { endpoint, user, .. } = setup().await.unwrap();
    let _agit_id = user.agits.first().unwrap().id;
    let client = Collection::get_client(&endpoint);
    let result = client
        .create(
            "TEST_Collection".to_string(),
            "TEDESCRIPTION".to_string(),
            None,
            IMAGE_URL.to_string(),
            IMAGE_URL.to_string(),
        )
        .await;

    assert!(
        result.is_ok(),
        "Failed to create collection: {:?}",
        result.err()
    );
    result.unwrap()
}

#[tokio::test]
async fn test_query_collection() {
    let TestContext { endpoint, user, .. } = setup().await.unwrap();
    let _agit_id = user.agits.first().unwrap().id;

    let client = Collection::get_client(&endpoint);

    let param = CollectionQuery {
        bookmark: None,
        size: 10,
    };
    let result = client.query(param).await;

    assert!(
        result.is_ok(),
        "Failed to query collection: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_delete_collection() {
    let TestContext { endpoint, user, .. } = setup().await.unwrap();
    let _agit_id = user.agits.first().unwrap().id;
    let client = Collection::get_client(&endpoint);
    let result = client.delete(1).await;

    assert!(
        result.is_ok(),
        "Failed to delete collection: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_update_collection() {
    let TestContext { endpoint, user, .. } = setup().await.unwrap();
    let _agit_id = user.agits.first().unwrap().id;
    let collection = test_create_collection();
    let client = Collection::get_client(&endpoint);

    let result = client
        .update(
            collection.id,
            collection.title,
            collection.description,
            None,
            collection.banner_url,
            collection.logo_url,
            true,
        )
        .await;

    assert!(
        result.is_ok(),
        "Failed to update collection: {:?}",
        result.err()
    );
    let result = result.unwrap();
    assert_eq!(result.authorized, true);
}
