#[cfg(test)]

    use super::*;
    use crate::dagit_tests::{TestContext, setup_test_db, setup};
    use models::{
        Result, error as ApiError,
        v1::collection::{CollectionCreateRequest, CollectionQuery, CollectionUpdateRequest},
    };

    #[tokio::test]
    async fn test_create_collection_unauthorized() {
        let TestContext {
            now,
            endpoint,
            ..
        } = setup().await.unwrap();
        let pool = setup_test_db().await;
        let controller = CollectionControllerV1::new(pool);

        let result = controller
            .create(
                None, 
                1, 
                CollectionCreateRequest {
                    title: "New Collection Title".to_string(),
                },
            )
            .await;

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), ApiError::ServiceError::Unauthorized);
    }

    #[tokio::test]
    async fn test_query_collection() {
        let pool = setup_test_db().await;
        let controller = CollectionControllerV1::new(pool);

        let param = CollectionQuery {
            bookmark: None,
            size: 10,
        };
        let result = controller.query(None, param).await;

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), ApiError::ServiceError::Unauthorized);
    }

    #[tokio::test]
    async fn test_update_collection() {
        let pool = setup_test_db().await;
        let controller = CollectionControllerV1::new(pool);

        let param = CollectionUpdateRequest {
            title: "Updated Collection Title".to_string(),
        };
        let result = controller.update(None, 1, param).await;

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), ApiError::ServiceError::Unauthorized);
    }

    #[tokio::test]
    async fn test_delete_collection() {
        let pool = setup_test_db().await;
        let controller = CollectionControllerV1::new(pool);

        let result = controller.delete(1, None).await;

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), ApiError::ServiceError::Unauthorized);
    }
