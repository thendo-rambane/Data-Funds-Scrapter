#[derive(Default, Debug)]
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
        let mut fees_and_costs = FeesAndCosts::new();
        fn process_value(value: &str) -> f32 {
            if let Ok(fee) =
                value.split('%').collect::<Vec<_>>()[0].parse::<f32>()
            {
                fee
            } else {
                0.0
            }
        }
        for (key, value) in hash_map.iter() {
            match key.as_str() {
                "Annual Management Fee" => {
                    fees_and_costs.annual_management_fee = process_value(value)
                }
                "Total TER" => fees_and_costs.total_ter = process_value(value),
                "Exit Fee" => fees_and_costs.exit_fee = process_value(value),
                "Trailer Fee" => {
                    fees_and_costs.trailer_fee = process_value(value)
                }
                "TER Performance Fee" => {
                    fees_and_costs.ter_performance_fee = process_value(value)
                }
                "Initial Fee" => {
                    fees_and_costs.initial_fee = process_value(value)
                }
                "Advisor Annual Fee" => {
                    fees_and_costs.advisor_annual_fee = process_value(value)
                }
                "TER" => fees_and_costs.ter = process_value(value),
                "Transaction Costs Ratio" => {
                    fees_and_costs.transaction_costs_ratio =
                        process_value(value)
                }
                "Advisor Initial Fee" => {
                    fees_and_costs.advisor_initial_fee = process_value(value)
                }
                "Performance Fee" => {
                    fees_and_costs.performance_fee = value.trim().into()
                }
                _ => {}
            }
        }
        fees_and_costs
    }
}
