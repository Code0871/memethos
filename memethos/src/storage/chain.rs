use uuid7::{Uuid, uuid7};
use std::{collections::HashMap};

#[derive(Debug, Clone)]
pub struct Chain {
    pub uuid: Uuid,
    pub vector: Vec<f64>,
    pub moment_ids: Vec<Uuid>,
    pub payload: HashMap<String, String>,
}

impl Chain {
    pub fn new(vector: Vec<f64>, moment_ids: Vec<Uuid>, payload: HashMap<String, String>) -> Self {
        Self {
            uuid: uuid7(),
            vector,
            moment_ids,
            payload,
        }
    }
}