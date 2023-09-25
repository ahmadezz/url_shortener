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
        .await?
    {
        Some(entry) => {
            // get the old visits count
            let count = entry.visits_count;
            // create active model from stats entry model
            let mut active_model_entry: entity::stats::ActiveModel = entry.into();
            // increment visits_count
            active_model_entry.visits_count = Set(count + 1);
            let _ = active_model_entry.update(db).await;
            Ok(())
        }
        None => bail!("Failed to find entry for id '{id}' in stats table to increase visits_count"),
    }
}
