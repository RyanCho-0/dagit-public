#![cfg(test)]


use crate::controllers::v1::agit::AgitControllerV1;
use crate::dagit_tests::{TestContext, setup, setup_test_db};
use models::{
    error as ApiError,
    v1::agit::{AgitCreateRequest, AgitQuery},
};

#[tokio::test]
async fn test_create_agit() {
    let TestContext {
        ..
    } = setup().await.unwrap();
    let pool = setup_test_db().await;
    let controller = AgitControllerV1::new(pool);

    let result = controller
        .create(
            None,
            AgitCreateRequest {
                title: "New Agit Title".to_string(),
            },
        )
        .await;

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), ApiError::ServiceError::Unauthorized);
}

#[tokio::test]
async fn test_query_agit() {
    let pool = setup_test_db().await;
    let controller = AgitControllerV1::new(pool);

    let param = AgitQuery {
        bookmark: None,
        size: 10,
    };
    let result = controller.query(None, param).await;

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), ApiError::ServiceError::Unauthorized);
}

#[tokio::test]
async fn test_delete_agit() {
    let pool = setup_test_db().await;
    let controller = AgitControllerV1::new(pool);

    let result = controller.delete(1, None).await;

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), ApiError::ServiceError::Unauthorized);
}


