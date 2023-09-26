use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use rand::Rng;
use sea_orm::DatabaseConnection;
use tracing::error;
use url::Url as url_parser;

use crate::{
    data::{find_id, find_url, increment_visits_count, insert_stats_entry, insert_urls_entry},
    model::{AppState, URLDBEntry, Url},
};

/// Parses the input to a valid url entry. Checks urls table for exisiting entries
/// for early id return if not found then it generates a unique id and inserts
/// the url as well as id into the urls table and stats table in the database
pub async fn get_short_url(
    State(state): State<AppState>,
    Json(url): Json<Url>,
) -> impl IntoResponse {
    // parse input into valid long url entry
    let parsed_url = match url_parser::parse(&url.long_url) {
        Ok(url) => url,
        Err(err) => {
            error!(
                "Failed to parse '{}' into a valid url due to: {}",
                url.long_url, err
            );
            return Err((StatusCode::UNPROCESSABLE_ENTITY, err.to_string()));
        }
    };

    // check for early return if long url already exists in urls table
    if let Some(id) = find_id(&state.db, parsed_url.as_str()).await {
        return Ok((StatusCode::OK, format!("{}{}", state.base_url, id)));
    };

    let id = match create_id(&state.db).await {
        Some(id) => id,
        None => {
            error!("Failed to get a unique short url due to a collision");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Short Url id collision error".to_string(),
            ));
        }
    };
    let entry = URLDBEntry::from((id.as_str(), parsed_url.as_str()));

    // insert long url and short url id into urls table
    if let Err(err) = insert_urls_entry(&state.db, entry.clone()).await {
        error!("Failed to insert urls entry due to:{}", err);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()));
    };

    // insert short url id into stats table
    if let Err(err) = insert_stats_entry(&state.db, entry).await {
        error!("Failed to insert stats entry due to:{}", err);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()));
    };
    Ok((StatusCode::OK, format!("{}{}", state.base_url, id)))
}

/// Finds equivalent url for the given id in the database.
/// Then increases the visits_count by 1 and redirects the request
pub async fn redirect_url(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // get long url from urls table in the database
    let url = match find_url(&state.db, &id).await {
        Some(url) => url,
        None => {
            error!("Failed to find url for id '{id}' in urls database");
            return Err(StatusCode::NOT_FOUND);
        }
    };

    // increment visits_count  in stats table and redirect to long url
    match increment_visits_count(&state.db, &id).await {
        Ok(_) => Ok(Redirect::to(&url)),
        Err(err) => {
            error!("Failed to increment visits_count due to:{}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Generates unique id string
pub async fn create_id(db: &DatabaseConnection) -> Option<String> {
    // generate random length between 1 and 7
    let random_length: usize = rand::thread_rng().gen_range(1..=7);
    // create a unique id entry with the randomly generated length
    let id = nanoid::nanoid!(random_length, &nanoid::alphabet::SAFE);
    // check for ids collision
    if find_id(db, &id).await.is_none() {
        return Some(id);
    }
    None
}
