use axum_macros::FromRef;
use serde::{Deserialize, Serialize};

// the application state
#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    // app config layer
    pub base_url: String,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Url {
    pub long_url: String,
    pub short_url: String,
}
