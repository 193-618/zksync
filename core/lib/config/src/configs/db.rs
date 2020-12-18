// External uses
use serde::Deserialize;
// Local uses
use crate::envy_load;

/// Used database configuration.
#[derive(Debug, Deserialize)]
pub struct DBConfig {
    /// Amount of open connections to the database held by server in the pool.
    pub pool_size: usize,
    /// Database URL.
    pub url: String,
}

impl DBConfig {
    pub fn from_env() -> Self {
        envy_load!("db", "DB_")
    }
}
