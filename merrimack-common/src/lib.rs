use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub interval_minutes: u64,
    pub duration_seconds: u64,
}
