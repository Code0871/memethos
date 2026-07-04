use uuid7::{Uuid, uuid7};
use std::{collections::HashMap};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Moment {
    pub uuid: Uuid,
    pub vector: Vec<f64>,
    pub timestamp: DateTime<Utc>,
    pub payload: HashMap<String, String>,
    pub next_moment_ids: Vec<Uuid>,
    pub prev_moment_ids: Vec<Uuid>,
}

impl Moment {
    pub fn new(vector: Vec<f64>, payload: HashMap<String, String>, next_moment_ids: Vec<Uuid>, prev_moment_ids: Vec<Uuid>) -> Self {
        Self {
            uuid: uuid7(),
            vector,
            timestamp: Utc::now(),
            payload,
            next_moment_ids,
            prev_moment_ids,
        }
    }
}