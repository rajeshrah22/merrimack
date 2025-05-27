use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub interval_minutes: u64,
    pub duration_seconds: u64,
}
