use anyhow::{bail, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tracing::{debug, error};

use crate::model::URLDBEntry;

/// Finds id from urls table for the given long url input
pub async fn find_id(db: &DatabaseConnection, url: &str) -> Option<String> {
    match entity::urls::Entity::find()
        .filter(entity::urls::Column::Url.eq(url))
        .one(db)
        .await
    {
        // return only the id
        Ok(Some(entry)) => Some(entry.id),
        Ok(None) => {
            debug!("Failed to find entries for url '{url}' in urls table");
            None
        }
        Err(err) => {
            error!("Failed to get id for url '{url}' due to:{err}");
            None
        }
    }
}

/// Finds full long url from urls table for the given short url id input
pub async fn find_url(db: &DatabaseConnection, id: &str) -> Option<String> {
    match entity::urls::Entity::find()
        .filter(entity::urls::Column::Id.eq(id))
        .one(db)
        .await
    {
        // return only the url
        Ok(Some(entry)) => Some(entry.url),
        Ok(None) => {
            debug!("Failed to find entries for id '{id}' in urls table");
            None
        }
        Err(err) => {
            error!("Failed to get url for id '{id}' due to:{err}");
            None
        }
    }
}

/// Inserts new short url id and its equivalent long url into urls table in the database
pub async fn insert_urls_entry(db: &DatabaseConnection, entry: URLDBEntry) -> Result<()> {
    // create active model with data mapping
    let url_entry = entity::urls::ActiveModel {
        id: Set(entry.id),
        url: Set(entry.long_url),
    };

    // insert urls entry
    if let Err(err) = entity::urls::Entity::insert(url_entry).exec(db).await {
        bail!("Failed to insert urls entry due to: {}", err);
    }
    Ok(())
}

/// Inserts new short url id and sets visits count to 0 into stats table in the database
pub async fn insert_stats_entry(db: &DatabaseConnection, entry: URLDBEntry) -> Result<()> {
    // create active model with data mapping
    let stats_entry = entity::stats::ActiveModel {
        id: Set(entry.id),
        ..Default::default()
    };

    // insert stats entry
    if let Err(err) = entity::stats::Entity::insert(stats_entry).exec(db).await {
        bail!("Failed to insert stats entry due to: {}", err);
    }

    Ok(())
}
/// Updates visits_count in stats table for the given id if found
pub async fn increment_visits_count(db: &DatabaseConnection, id: &str) -> Result<()> {
    // get stats entry model
    match entity::stats::Entity::find()
        .filter(entity::stats::Column::Id.eq(id))
        .one(db)
        .await
    {
        Ok(Some(entry)) => {
            // get the old visits count
            let count = entry.visits_count;
            // create active model from stats entry model
            let mut active_model_entry: entity::stats::ActiveModel = entry.into();
            // increment visits_count
            active_model_entry.visits_count = Set(count + 1);
            let _ = active_model_entry.update(db).await;
            Ok(())
        }
        Ok(None) => {
            bail!("Failed to find entry for id '{id}' in stats table to increase visits_count")
        }
        Err(err) => bail!("Failed to find entry in status table due to: {err}"),
    }
}
#[cfg(test)]
mod tests {
    use sea_orm::{entity::prelude::*, DatabaseBackend, MockDatabase, MockExecResult, Transaction};

    use crate::{
        data::{find_url, increment_visits_count, insert_stats_entry, insert_urls_entry},
        model::URLDBEntry,
    };

    use super::find_id;

    #[tokio::test]
    async fn test_find_id() -> Result<(), DbErr> {
        // Create MockDatabase with mock query results
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![
                // First query result
                vec![entity::urls::Model {
                    id: "x10N".to_string(),
                    url: "www.google.com".to_string(),
                }],
                // Second query result
                vec![],
            ])
            .into_connection();
        assert_eq!(
            find_id(&db, &"www.google.com").await,
            Some("x10N".to_string())
        );
        assert_eq!(find_id(&db, &"www.yahoo.com").await, None);

        Ok(())
    }
    #[tokio::test]
    async fn test_find_id_db_timeout_err() -> Result<(), DbErr> {
        // Create MockDatabase with mock query results
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([DbErr::ConnectionAcquire(ConnAcquireErr::Timeout)])
            .into_connection();
        assert_eq!(find_id(&db, &"www.google.com").await, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_find_url() -> Result<(), DbErr> {
        // Create MockDatabase with mock query results
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![
                // First query result
                vec![entity::urls::Model {
                    id: "x10N".to_string(),
                    url: "www.google.com".to_string(),
                }],
                // Second query result
                vec![],
            ])
            .into_connection();
        assert_eq!(
            find_url(&db, &"x10N").await,
            Some("www.google.com".to_string())
        );
        assert_eq!(find_url(&db, &"ru6UN1").await, None);

        Ok(())
    }
    #[tokio::test]
    async fn test_find_url_db_connection_err() -> Result<(), DbErr> {
        // Create MockDatabase with mock query results
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([DbErr::ConnectionAcquire(ConnAcquireErr::ConnectionClosed)])
            .into_connection();
        assert_eq!(find_url(&db, &"ru6UN1").await, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_insert_urls_entry() -> Result<(), DbErr> {
        // Create MockDatabase with mock query results
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results([MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();
        let urls_entry = URLDBEntry {
            id: "x10N".to_string(),
            long_url: "www.google.com".to_string(),
        };
        let result = insert_urls_entry(&db, urls_entry).await;
        assert!(result.is_ok());
        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"INSERT INTO "urls" ("id", "url") VALUES ($1, $2) RETURNING "id""#,
                ["x10N".into(), "www.google.com".into()]
            )]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_insert_urls_entry_connection_err() -> Result<(), DbErr> {
        // Create MockDatabase with mock query results
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_errors([DbErr::ConnectionAcquire(ConnAcquireErr::ConnectionClosed)])
            .into_connection();
        let urls_entry = URLDBEntry {
            id: "x10N".to_string(),
            long_url: "www.google.com".to_string(),
        };
        let result = insert_urls_entry(&db, urls_entry).await;
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(),"Failed to insert urls entry due to: Failed to acquire connection from pool: Connection closed".to_string());
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_insert_stats_entry() -> Result<(), DbErr> {
        // Create MockDatabase with mock query results
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results([MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();
        let stats_entry = URLDBEntry {
            id: "x10N".to_string(),
            long_url: "www.google.com".to_string(),
        };
        let result = insert_stats_entry(&db, stats_entry).await;
        assert!(result.is_ok());
        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"INSERT INTO "stats" ("id") VALUES ($1) RETURNING "id""#,
                ["x10N".into()]
            )]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_insert_stats_entry_timeout_err() -> Result<(), DbErr> {
        // Create MockDatabase with mock query results
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_errors([DbErr::ConnectionAcquire(ConnAcquireErr::Timeout)])
            .into_connection();
        let stats_entry = URLDBEntry {
            id: "x10N".to_string(),
            long_url: "www.google.com".to_string(),
        };
        let result = insert_stats_entry(&db, stats_entry).await;
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(),"Failed to insert stats entry due to: Failed to acquire connection from pool: Connection pool timed out".to_string());
        };

        Ok(())
    }

    #[tokio::test]
    async fn test_increment_visits_count() -> Result<(), DbErr> {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![entity::stats::Model {
                id: "x10N".to_string(),
                visits_count: 0,
            }]])
            .into_connection();
        assert!(increment_visits_count(&db, "x10N").await.is_ok());

        assert_eq!(
            db.into_transaction_log(),
            [
                Transaction::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    r#"SELECT "stats"."id", "stats"."visits_count" FROM "stats" WHERE "stats"."id" = $1 LIMIT $2"#,
                    ["x10N".into(), (1 as u64).into()]
                ),
                Transaction::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    r#"UPDATE "stats" SET "visits_count" = $1 WHERE "stats"."id" = $2 RETURNING "id", "visits_count""#,
                    [1.into(), "x10N".into()]
                )
            ]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_increment_visits_count_timeout_err() -> Result<(), DbErr> {
        // Create MockDatabase with mock query results
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([DbErr::ConnectionAcquire(ConnAcquireErr::Timeout)])
            .into_connection();

        let result = increment_visits_count(&db, "x10N").await;
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(),"Failed to find entry in status table due to: Failed to acquire connection from pool: Connection pool timed out".to_string());
        };
        Ok(())
    }
}
