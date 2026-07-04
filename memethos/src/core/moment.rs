use uuid::Uuid;
use std::{collections::HashMap};

struct Moment {
    pub moment_uuid: Uuid,
    pub moment_vector: Vec<f64>,
    pub moment_data: timestamp::DateTime<timestamp::Utc>,
    pub moment_payload: HashMap<String, String>,
}