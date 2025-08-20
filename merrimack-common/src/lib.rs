use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub interval_minutes: i64,
    pub duration_seconds: i64,
}
