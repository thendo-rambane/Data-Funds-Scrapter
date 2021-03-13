use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Returns {
    three_months: Option<f32>,
    six_months: Option<f32>,
    one_year: Option<f32>,
    three_years: Option<f32>,
    five_years: Option<f32>,
    ten_years: Option<f32>,
}

impl Returns {
    pub fn new() -> Self {
        Returns {
            three_months: None,
            six_months: None,
            one_year: None,
            three_years: None,
            five_years: None,
            ten_years: None,
        }
    }
    pub fn from_hash_map(
        hash_map: &std::collections::HashMap<String, Option<f32>>,
    ) -> Self {
        let temp = json!({
            "three_months": &hash_map["3M"],
            "six_months": &hash_map["6M"],
            "one_year": &hash_map["1Y"],
            "three_years": &hash_map["3Y"],
            "five_years": &hash_map["5Y"],
            "ten_years": &hash_map["10Y"],
        });
        serde_json::from_value(temp).unwrap()
    }
}
