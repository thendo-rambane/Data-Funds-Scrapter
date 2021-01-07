use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct StatutoryData {
    fax: String,
    telephone: String,
    directors: String,
    trustee: String,
    registration_number: String,
    postal_address: String,
    physical_address: String,
    sponsors: String,
    auditors: String,
    management_company: String,
    advisors: String,
    tollfree: String,
    website: String,
    email: String,
}

impl StatutoryData {
    pub fn new() -> Self {
        StatutoryData {
            fax: String::new(),
            telephone: String::new(),
            directors: String::new(),
            trustee: String::new(),
            registration_number: String::new(),
            postal_address: String::new(),
            physical_address: String::new(),
            sponsors: String::new(),
            auditors: String::new(),
            management_company: String::new(),
            advisors: String::new(),
            tollfree: String::new(),
            website: String::new(),
            email: String::new(),
        }
    }

    pub fn from_hash_map(
        hash_map: &std::collections::HashMap<String, String>,
    ) -> Self {
        let temp = json!({
            "fax": hash_map["Fax"],
            "telephone": hash_map["Telephone"],
            "directors": hash_map["Directors"],
            "trustee": hash_map["Trustee"],
            "registration_number": hash_map["Registration Number"],
            "postal_address": hash_map["Postal Address"],
            "physical_address": hash_map["Physical Address"],
            "sponsors": hash_map["Sponsors"],
            "auditors": hash_map["Auditors"],
            "management_company": hash_map["Management Company"],
            "advisors": hash_map["Advisors"],
            "tollfree": hash_map["Tollfree"],
            "website": hash_map["Website"],
            "email": hash_map["Email"],
        });
        serde_json::from_value(temp).unwrap()
    }
}
