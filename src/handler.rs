use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::DatabaseConnection;
use tracing::error;
use url::Url as url_parser;

use crate::model::{AppState, Url};

/// Parses the input to a valid url entry. Checks urls table for exisiting entries
/// for early id return if not found then it generates a unique id and inserts
/// the url as well as id into the urls table and stats table in the database
pub async fn get_short_url(
    State(state): State<AppState>,
    Json(url): Json<Url>,
) -> impl IntoResponse {
    // parse input into valid long url entry
    let _parsed_url = match url_parser::parse(&url.long_url) {
        Ok(url) => url,
        Err(err) => {
            error!(
                "The provided input '{}' couldn't be parsed into a valid url due to: {}",
                url.long_url, err
            );
            return Err((StatusCode::UNPROCESSABLE_ENTITY, err.to_string()));
        }
    };

    Ok((StatusCode::OK, format!("{}{}", state.base_url, "id")))
}

/// Finds equivalent url for the given id in the database.
/// Then increases the visits_count by 1 and redirects the request
pub async fn redirect_url(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
) -> impl IntoResponse {
    // get long url from urls table in the database
    // increment visits_count  in stats table and redirect to long url
    todo!();
}

/// Generates unique id string
pub async fn _create_id(_db: &DatabaseConnection) -> Option<String> {
    // generate random length between 1 and 11
    // create a unique id entry with the randomly generated length
    // check for ids collision
    todo!();
}
