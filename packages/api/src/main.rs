pub mod config;
pub mod controllers;

use by_axum::{auth::authorization_middleware, axum::middleware};
use controllers::v1;

use by_types::DatabaseConfig;
use models::Result;
use models::v1::{agit::Agit, artist::Artist, artwork::Artwork, collection::Collection};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod utils;

async fn migration(pool: &sqlx::Pool<sqlx::Postgres>) -> models::Result<()> {
    //TODO: Add Model Migration
    let agit = Agit::get_repository(pool.clone());
    let collection = Collection::get_repository(pool.clone());
    let artwork = Artwork::get_repository(pool.clone());

    let artist = Artist::get_repository(pool.clone());

    agit.create_this_table().await?;
    collection.create_this_table().await?;
    artist.create_this_table().await?;
    artwork.create_this_table().await?;

    agit.create_related_tables().await?;
    collection.create_related_tables().await?;
    artist.create_related_tables().await?;
    artwork.create_related_tables().await?;

    Ok(())
}
    

#[tokio::main]
async fn main() -> models::Result<()> {
    let app = by_axum::new();
    let conf = config::get();
    tracing::debug!("config: {:?}", conf);

    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await
            .expect("Failed to connect to Postgres")
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    migration(&pool).await?;

    let app = app
        .nest("/v1", v1::routes(pool.clone())?)
        .layer(middleware::from_fn(authorization_middleware));
    let port = option_env!("PORT").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[cfg(test)]
pub mod dagit_tests {
    use std::{collections::HashMap, time::SystemTime};
    use by_types::Claims;
    use rest_api::ApiService;

    use super::*;

    pub struct TestContext {
        pub pool: sqlx::Pool<sqlx::Postgres>,
        pub app: Box<dyn ApiService>,
        pub agit_id: i64,
        // pub user: User,
        pub user_token: String,
        pub now: i64,
        pub id: String,
        pub claims: Claims,
        pub endpoint: String,
    }

    pub struct User {
        pub id: i64,
        pub email: String,
        pub password: String,
    }

    pub async fn setup_test_db() -> sqlx::Pool<sqlx::Postgres> {
        let conf = config::get();
        if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
            PgPoolOptions::new()
                .max_connections(pool_size)
                .connect(url)
                .await
                .expect("Failed to connect to Postgres")
        } else {
            panic!("Database is not initialized. Call init() first.");
        }
    }

    pub fn setup_jwt_token(user: User) -> (Claims, String) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut claims = Claims {
            sub: user.id.to_string(),
            exp: now + 3600,
            role: by_types::Role::User,
            custom: HashMap::new(),
        };
        let token = by_axum::auth::generate_jwt(&mut claims).unwrap();
        (claims, token)
    }

    pub async fn setup() -> Result<TestContext> {
        let conf = config::get();
        let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
            PgPoolOptions::new()
                .max_connections(pool_size)
                .connect(url)
                .await
                .expect("Failed to connect to Postgres")
        } else {
            panic!("Database is not initialized. Call init() first.");
        };

        // Run necessary SQL setup queries
        sqlx::query("CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at := EXTRACT(EPOCH FROM now()); RETURN NEW; END; $$ LANGUAGE plpgsql;")
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query("CREATE OR REPLACE FUNCTION set_created_at() RETURNS TRIGGER AS $$ BEGIN NEW.created_at := EXTRACT(EPOCH FROM now()); RETURN NEW; END; $$ LANGUAGE plpgsql;")
        .execute(&pool)
        .await
        .unwrap();

        let app = by_axum::new();
        let app = by_axum::into_api_adapter(app);

        let id = uuid::Uuid::max();
        let user = User {
            id: 1,
            email: "j@ff.com".to_string(),
            password: "password".to_string(),
        };

        let (claims, user_token) = setup_jwt_token(user);

        let app = Box::new(app);

        rest_api::set_api_service(app.clone());
        rest_api::add_authorization(&format!("Bearer {}", user_token));
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        Ok(TestContext {
            pool,
            app,
            id: id.to_string(),
            user_token,
            claims,
            now: now as i64,
            endpoint: "http://localhost:3000".to_string(),
            agit_id: 1,
        })
    }
}
