use uuid::Uuuid;
use std::{collections::HashMap};

// Chain struct
pub struct Chain {
    pub chain_id: Uuid,                             // chain unique identifier
    pub chain_vector: Vec<f64>,                     // chain vector representation
    pub chain_moment_ids: Vec<Uuid>,                // list of moment identifiers
    pub chain_payload: HashMap<String, String>,     // additional chain data
}