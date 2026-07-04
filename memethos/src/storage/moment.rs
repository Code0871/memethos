use std::{collections::HashMap};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Moment {
    pub uuid: uuid::Uuid,
    pub chain_id: uuid::Uuid,
    pub vector: Vec<f64>,
    pub timestamp: DateTime<Utc>,
    pub payload: HashMap<String, String>,
    pub next_moment_ids: Option<Vec<Uuid>>,
    pub prev_moment_ids: Option<Vec<Uuid>>,
}

pub struct MomentBuilder {
    uuid: Option<Uuid>,
    chain_id: Uuid,
    vector: Vec<f64>,
    timestamp: DateTime<Utc>,
    payload: HashMap<String,String>,
    next_moments_ids: Option<Vec<Uuid>>,
    prev_moments_ids: Option<Vec<Uuid>>,
}

impl MomentBuilder {
    pub fn new(chain_id: Uuid, vector: Vec<f64>) -> Self{
        Self{
            uuid: None,
            chain_id,
            vector,
            timestamp: chrono::Utc::now(),
            payload: HashMap::new(),
            next_moments_ids: None,
            prev_moments_ids: None,
        }
    }

    pub fn with_guid(mut self, guid: Uuid) -> Self {
        self.uuid = Some(guid);
        self
    }

    pub fn with_payload(mut self, payload: HashMap<String,String>) -> Self{
        self.payload=payload;
        self
    }

    pub fn with_next_moments(mut self, next_node: Vec<Uuid>) -> Self {
        self.next_moments_ids = Some(next_node);
        self
    }

    pub fn with_prev_moments(mut self, prev_node: Vec<Uuid>) -> Self {
        self.prev_moments_ids = Some(prev_node);
        self
    }

    pub fn build(self) -> Result<Moment, String> {
        if self.vector.is_empty() {
           return Err("Vector is empty".to_string())
        }

        Ok(Moment {
            uuid: self.uuid.unwrap_or(Uuid::now_v7()),
            chain_id: self.chain_id,
            vector: self.vector,
            timestamp: self.timestamp,
            payload: self.payload,
            next_moment_ids: self.next_moments_ids,
            prev_moment_ids: self.prev_moments_ids,
        })
    }

}


impl Moment {
    pub fn builder(chain_id: Uuid, vector: Vec<f64>) -> MomentBuilder {
        MomentBuilder::new(chain_id, vector)
    }
}