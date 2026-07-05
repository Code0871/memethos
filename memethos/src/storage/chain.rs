    use uuid::Uuid;
    use std::{collections::HashMap};

    #[derive(Debug, Clone)]
    pub struct Chain {
        pub uuid: Uuid,
        pub vector: Vec<f32>,
        pub moment_ids: Vec<Uuid>,
        pub payload: HashMap<String, String>,
        pub is_deleted: bool,
    }

    #[derive(Debug, Clone, Default)]
    pub struct ChainBuilder  {
        uuid : Option<Uuid>,
        vector: Vec<f32>,
        moment_ids: Vec<Uuid>,
        payload :HashMap <String,String>,
        is_deleted: bool
    }

    impl ChainBuilder {

        // Default constructor
        pub fn new(vector: Vec<f32>) -> Self {
            Self {
                uuid: None,
                vector,
                moment_ids: Vec::new(),
                payload: HashMap::new(),
                is_deleted: false,
            }
        }

        // Getter for the GUID
        pub fn with_guid(mut self, guid: Uuid) -> Self {
            self.uuid = Some(guid);
            self
        }

        // Getter for the payload
        pub fn with_payload(mut self, payload: HashMap<String, String>) -> Self {
            self.payload = payload;
            self
        }

        pub fn with_moments(mut self, moment_ids: Vec<Uuid>) -> Self {
            self.moment_ids = moment_ids;
            self
        }

        // Getter for the deleted flag
        pub fn is_deleted(mut self, is_deleted: bool) -> Self {
            self.is_deleted = is_deleted;
            self
        }

        pub fn build(self) -> Result<Chain, String> {

            if self.vector.is_empty() {
                return Err("Vector cannot be empty!!".to_string());
            }
            
            Ok(Chain {
                uuid: self.uuid.unwrap_or_else(Uuid::now_v7),
                vector: self.vector,
                moment_ids: self.moment_ids,
                payload: self.payload,
                is_deleted: self.is_deleted,
            })
        }
    }

    impl Chain {    
        pub fn builder(vector: Vec<f32>) -> ChainBuilder {
            ChainBuilder::new(vector)
        }
    }

