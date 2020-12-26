mod fees_and_costs;
mod statutory_info;
use fees_and_costs::FeesAndCosts;
use statutory_info::StatutoryData;

#[derive(Debug)]
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
    ) -> Self {
        let mut unit_trust = UnitTrust::new();
        fn process_value(value: &str) -> usize {
            value
                .trim()
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .unwrap_or(0)
        }
        for (key, value) in hash_map.iter() {
            match key.as_str() {
                "Formation Date" => {
                    unit_trust.formation_date = value.trim().into();
                }
                "Fund Size" => {
                    unit_trust.fund_size = process_value(value);
                }
                "Domicile" => unit_trust.domicile = value.trim().into(),
                "Reporting Currency" => {
                    unit_trust.reporting_currency = value.trim().into()
                }
                "JSE Code" => unit_trust.jse_code = value.trim().into(),
                "ISIN" => unit_trust.isin = value.trim().into(),
                "Number of unitholders" => {
                    unit_trust.number_of_unit_holders =
                        process_value(value) as u32
                }
                "Pricing" => unit_trust.pricing = value.trim().into(),
                "ASISA Category" => {
                    unit_trust.asisa_category = value.trim().into()
                }
                "Benchmark" => unit_trust.benchmark = value.trim().into(),
                "Income Distributions" => {
                    unit_trust.income_distributions = value.trim().into()
                }
                "Income Payment" => {
                    unit_trust.income_payment = value.trim().into()
                }
                "Fund Management" => {
                    unit_trust.fund_management = value.trim().into()
                }
                "Minimum Investment" => {
                    unit_trust.minimum_investment = process_value(value) as u32
                }
                "Minimum Top-Up" => {
                    unit_trust.minimum_top_up = process_value(value) as u32
                }
                "Minimum Monthly" => {
                    unit_trust.minimum_monthly = process_value(value) as u32
                }
                "Risk Rating" => unit_trust.risk_rating = value.trim().into(),
                "name" => unit_trust.name = value.trim().into(),
                "reg 28 compliant" => {
                    unit_trust.is_reg_28_comliant = value.as_str() == "true"
                }
                _ => {}
            }
        }
        unit_trust
    }
}

impl Default for UnitTrust {
    fn default() -> Self {
        UnitTrust::new()
    }
}
