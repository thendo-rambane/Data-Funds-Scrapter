use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FeesAndCosts {
    advisor_initial_fee: f32,
    transaction_costs_ratio: f32,
    exit_fee: f32,
    advisor_annual_fee: f32,
    trailer_fee: f32,
    total_ter: f32,
    ter: f32,
    performance_fee: String,
    annual_management_fee: f32,
    initial_fee: f32,
    ter_performance_fee: f32,
}

impl FeesAndCosts {
    pub fn new() -> Self {
        FeesAndCosts {
            advisor_initial_fee: 0.0,
            transaction_costs_ratio: 0.0,
            exit_fee: 0.0,
            advisor_annual_fee: 0.0,
            trailer_fee: 0.0,
            total_ter: 0.0,
            ter: 0.0,
            performance_fee: "".into(),
            annual_management_fee: 0.0,
            initial_fee: 0.0,
            ter_performance_fee: 0.0,
        }
    }
    pub fn from_hash_map(
        hash_map: &std::collections::HashMap<String, String>,
    ) -> Self {
        fn process_value(value: &str) -> f32 {
            if let Ok(fee) =
                value.split('%').collect::<Vec<_>>()[0].parse::<f32>()
            {
                fee
            } else {
                0.0
            }
        }
        let temp = json!({
            "annual_management_fee":
                process_value(&hash_map["Annual Management Fee"]),
            "total_ter": process_value(&hash_map["Total TER"]),
            "advisor_initial_fee":
                process_value(&hash_map["Advisor Initial Fee"]),
            "transaction_costs_ratio":
                process_value(&hash_map["Transaction Costs Ratio"]),
            "exit_fee": process_value(&hash_map["Exit Fee"]),
            "advisor_annual_fee":
                process_value(&hash_map["Advisor Annual Fee"]),
            "trailer_fee": process_value( &hash_map[ "Trailer Fee" ] ),
            "ter": process_value( &hash_map[ "TER"] ),
            "performance_fee":
                String::from(hash_map[ "Performance Fee" ].trim()),
            "initial_fee": process_value( &hash_map[ "Initial Fee" ] ),
            "ter_performance_fee":
                process_value( &hash_map[ "TER Performance Fee" ] ),
        });
        serde_json::from_value(temp).unwrap()
    }
}
