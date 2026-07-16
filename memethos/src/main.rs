use std::collections::HashMap;
use memethos::Config;
use memethos::{Chain, Moment};
use memethos::estimate::estimate::*;
use uuid::Uuid;

fn main() {

    //###################################
    //####### TESTS #####################
    //###################################

    let per = Config::load_from_file();

    match per {
        Ok(config) => {
            println!("All is Ok");
            println!("{:?}", config);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            panic!();
        }
    }

    let specific_uuid = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();    
    
    let chain = Chain::builder(vec![1.5, 1.0353, 0.53783, 0.348])
        .with_guid(specific_uuid)
        .with_moments(vec![Uuid::now_v7()])
        .is_deleted(false)
        .build()
        .unwrap();

    println!("{:?}", chain);

    let payload: HashMap<String, String> = HashMap::from([("chain".to_string(), "Smth".to_string())]);
    
    let moment = Moment::builder(specific_uuid, vec![0.231, 0.67348, 0.97582, 1.35783])
        .with_payload(payload)
        .with_next_moments(vec![Uuid::now_v7()])
        .build()
        .unwrap();

    println!("{:?}", moment);

        let input = EstimateInput::new(
            768,                  // dimension
            1_000_000,          // vector_count
            DataType::I8,       // dtype (4 bytes)
            3,            // vectors_per_dot
        );

        estimate(input);
}
