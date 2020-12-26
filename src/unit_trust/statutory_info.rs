#[derive(Debug)]
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
        let mut stat_data = StatutoryData::new();
        for (key, value) in hash_map {
            match key.as_str() {
                "Fax" => stat_data.fax = value.trim().into(),
                "Telephone" => stat_data.telephone = value.trim().into(),
                "Directors" => stat_data.directors = value.trim().into(),
                "Trustee" => stat_data.trustee = value.trim().into(),
                "Registration Number" => {
                    stat_data.registration_number = value.trim().into()
                }
                "Postal Address" => {
                    stat_data.postal_address = value.trim().into()
                }
                "Physical Address" => {
                    stat_data.postal_address = value.trim().into()
                }
                "Sponsors" => stat_data.sponsors = value.trim().into(),
                "Auditors" => stat_data.advisors = value.trim().into(),
                "Management Company" => {
                    stat_data.management_company = value.trim().into()
                }
                "Advisors" => stat_data.advisors = value.trim().into(),
                "Tollfree" => stat_data.tollfree = value.trim().into(),
                "Website" => stat_data.website = value.trim().into(),
                "Email" => stat_data.email = value.trim().into(),
                _ => {}
            }
        }
        stat_data
    }
}
