mod unit_trust;
use unit_trust::services;
use unit_trust::UnitTrust;

fn get_trusts(url: &str) -> Vec<UnitTrust> {
    let unit_trusts = services::get_links(&url);
    unit_trusts
        .iter()
        // .take(1)
        .map(|link| {
            let url_query_fragment = link.split('?').collect::<Vec<_>>()[1];
            let fund_details_url =
                url.to_owned() + "FundDetails.aspx?" + url_query_fragment;
            let performance_url =
                url.to_owned() + "Performance.aspx?" + url_query_fragment;
            let details_document =
                services::get_document(&fund_details_url).unwrap();
            let returns_document =
                services::get_document(&performance_url).unwrap();
            let info = services::get_detailed_information(&details_document);
            let fees = services::get_fees_and_costs(&details_document);
            let statutory_info =
                services::get_statutory_data(&details_document);
            let returns = services::get_returns(&returns_document);
            UnitTrust::from_hash_map(&info, &fees, &statutory_info, &returns)
        })
        .collect::<Vec<UnitTrust>>()
}

fn main() {
    let base_url = String::from("https://www.fundsdata.co.za/fdov2/");
    print!("{}", serde_json::to_string(&get_trusts(&base_url)).unwrap());
}
