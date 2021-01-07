mod fees_and_costs;
mod statutory_info;
use fees_and_costs::FeesAndCosts;
use serde::{Deserialize, Serialize};
use serde_json::json;
use statutory_info::StatutoryData;

#[derive(Serialize, Deserialize, Debug)]
pub struct UnitTrust {
    formation_date: String,
    is_reg_28_comliant: bool,
    fund_size: usize,
    domicile: String,
    reporting_currency: String,
    jse_code: String,
    isin: String,
    pricing: String,
    number_of_unit_holders: u32,
    asisa_category: String,
    benchmark: String,
    name: String,
    income_distributions: String,
    income_payment: String,
    fund_management: String,
    minimum_investment: u32,
    minimum_top_up: u32,
    minimum_monthly: u32,
    risk_rating: String,
    fees_and_costs: FeesAndCosts,
    statutory_data: StatutoryData,
}

impl UnitTrust {
    pub fn new() -> Self {
        UnitTrust {
            formation_date: "".to_owned(),
            fund_size: 0,
            is_reg_28_comliant: false,
            domicile: "".to_owned(),
            name: "".to_owned(),
            reporting_currency: "".to_owned(),
            jse_code: "".to_owned(),
            isin: "".to_owned(),
            pricing: "".to_owned(),
            number_of_unit_holders: 0,
            asisa_category: "".to_owned(),
            benchmark: "".to_owned(),
            income_distributions: "".to_owned(),
            income_payment: "".to_owned(),
            fund_management: "".to_owned(),
            minimum_investment: 0,
            minimum_top_up: 0,
            minimum_monthly: 0,
            risk_rating: "".to_owned(),
            fees_and_costs: FeesAndCosts::new(),
            statutory_data: StatutoryData::new(),
        }
    }
    pub fn fees_and_costs_from_hash(
        &mut self,
        hash_map: &std::collections::HashMap<String, String>,
    ) {
        self.fees_and_costs = FeesAndCosts::from_hash_map(hash_map);
    }
    pub fn statutory_data_from_hash(
        &mut self,
        hash_map: &std::collections::HashMap<String, String>,
    ) {
        self.statutory_data = StatutoryData::from_hash_map(hash_map)
    }
    pub fn from_hash_map(
        hash_map: &std::collections::HashMap<String, String>,
        fees_hash_map: &std::collections::HashMap<String, String>,
        statutory_data_hash_map: &std::collections::HashMap<String, String>,
    ) -> Self {
        fn process_value(value: &str) -> usize {
            value
                .trim()
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .unwrap_or(0)
        }
        let value = json!({
            "formation_date": hash_map["Formation Date"],
            "is_reg_28_comliant": hash_map["reg 28 compliant"] == "true",
            "fund_size": process_value(&hash_map["Fund Size"]),
            "domicile": hash_map["Domicile"].trim(),
            "reporting_currency": hash_map["Reporting Currency"].trim(),
            "jse_code": hash_map["JSE Code"].trim(),
            "isin": hash_map["ISIN"].trim(),
            "pricing": hash_map["Pricing"].trim(),
            "number_of_unit_holders":
                process_value(&hash_map["Number of unitholders"]) as u32,
            "asisa_category": hash_map["ASISA Category"],
            "benchmark": hash_map["Benchmark"],
            "name": hash_map["name"],
            "income_distributions": hash_map["Income Distributions"].trim(),
            "income_payment": hash_map["Income Payment"].trim(),
            "fund_management": hash_map["Fund Management"].trim(),
            "minimum_investment":
                process_value(&hash_map["Minimum Investment"]) as u32,
            "minimum_top_up":
                process_value(&hash_map["Minimum Top-Up"]) as u32,
            "minimum_monthly":
                process_value(&hash_map["Minimum Monthly"]) as u32,
            "risk_rating": hash_map["Risk Rating"],
            "fees_and_costs": FeesAndCosts::from_hash_map(fees_hash_map),
            "statutory_data":
                StatutoryData::from_hash_map(statutory_data_hash_map),
        });
        serde_json::from_value(value).unwrap()
    }
}

impl Default for UnitTrust {
    fn default() -> Self {
        UnitTrust::new()
    }
}
