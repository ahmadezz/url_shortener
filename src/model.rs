use axum_macros::FromRef;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

// the application state
#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    // db connection layer
    pub db: DatabaseConnection,
    // app config layer
    pub base_url: String,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Url {
    pub long_url: String,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct URLDBEntry {
    pub id: String,
    pub long_url: String,
}
impl From<(&str, &str)> for URLDBEntry {
    fn from(body: (&str, &str)) -> Self {
        URLDBEntry {
            id: body.0.to_string(),
            long_url: body.1.to_string(),
        }
    }
}
