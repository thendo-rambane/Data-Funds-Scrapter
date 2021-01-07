mod unit_trust;
use unit_trust::UnitTrust;

fn get_document(
    url: &str,
) -> Result<scraper::html::Html, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).send()?.text()?;
    Ok(scraper::Html::parse_document(&response))
}
fn get_links(url: &str) -> Vec<String> {
    let base_url = String::from(url);
    let request_url = base_url + "Funds.aspx?menu=sector&filter=ZIVT";
    let document = get_document(&request_url).unwrap();
    let table_class = scraper::Selector::parse(".TableData").unwrap();
    let tbody_selector = scraper::Selector::parse("tbody").unwrap();
    let anchor_selector = scraper::Selector::parse("a").unwrap();

    document
        .select(&table_class)
        .next()
        .unwrap()
        .select(&tbody_selector)
        .next()
        .unwrap()
        .select(&anchor_selector)
        .filter(|anchor| anchor.value().attr("title").is_none())
        .map(|anchor| {
            if let Some(href) = anchor.value().attr("href") {
                String::from(href)
            } else {
                String::new()
            }
        })
        .collect()
}

fn get_fees_and_costs(
    document: &scraper::html::Html,
) -> std::collections::HashMap<String, String> {
    let detail_selector =
        scraper::Selector::parse("#FeesAndCosts_DivFundDetails").unwrap();
    let table_row_selector = scraper::Selector::parse("tr").unwrap();
    let cell_selector = scraper::Selector::parse("td").unwrap();
    let strong_selector = scraper::Selector::parse("strong").unwrap();
    document
        .select(&detail_selector)
        .next()
        .unwrap()
        .select(&table_row_selector)
        .skip(3)
        .map(|row| {
            let mut cells = row.select(&cell_selector);
            let key = if let Some(key_data) =
                cells.next().unwrap().select(&strong_selector).next()
            {
                key_data.text().collect::<String>()
            } else {
                "Error No Key".into()
            };

            let val = if let Some(value) = cells.next() {
                value.text().collect::<String>()
            } else {
                String::from("Error No Value")
            };
            (key, val)
        })
        .collect::<std::collections::HashMap<_, _>>()
}
fn get_statutory_data(
    document: &scraper::html::Html,
) -> std::collections::HashMap<String, String> {
    let detail_selector =
        scraper::Selector::parse("#StatutoryData_DivFundDetails").unwrap();
    let table_row_selector = scraper::Selector::parse("tr").unwrap();
    let cell_selector = scraper::Selector::parse("td").unwrap();
    document
        .select(&detail_selector)
        .next()
        .unwrap()
        .select(&table_row_selector)
        .skip(4)
        .map(|row| {
            let mut cells = row.select(&cell_selector);
            let key = if let Some(key_data) = cells.next() {
                key_data.text().collect::<String>().trim().into()
            } else {
                "Error No Key".into()
            };

            let val = if let Some(value) = cells.next() {
                value.text().collect::<String>()
            } else {
                String::from("Error No Value")
            };
            (key, val)
        })
        .collect::<std::collections::HashMap<_, _>>()
}
fn get_returns(
    document: &scraper::html::Html,
) -> std::collections::HashMap<String, String> {
    todo!("Get returns Data")
}
fn get_detailed_information(
    document: &scraper::html::Html,
    is_reg_28_comliant: bool,
) -> std::collections::HashMap<String, String> {
    let detail_selector =
        scraper::Selector::parse("#TechnicalDetails_DivFundDetails").unwrap();
    let p_selector = scraper::Selector::parse("p").unwrap();
    let strong_selector = scraper::Selector::parse("strong").unwrap();
    let table_row_selector = scraper::Selector::parse("tr").unwrap();
    let cell_selector = scraper::Selector::parse("td").unwrap();
    let name_selector =
        scraper::Selector::parse("#FundHeader1_LblFullname").unwrap();
    let name = document.select(&name_selector).next().unwrap().inner_html();
    document
        .select(&detail_selector)
        .next()
        .unwrap()
        .select(&table_row_selector)
        .skip(3)
        .map(|row| {
            let mut cells = row.select(&cell_selector);
            let key_data =
                cells.next().unwrap().select(&p_selector).next().unwrap();
            let mut key = key_data.inner_html();
            for key_value in key_data.select(&strong_selector) {
                if key_value.inner_html() != "" {
                    key = key_value.inner_html();
                }
            }
            let mut value_data = cells.next().unwrap().select(&p_selector);
            let val = if let Some(value) = value_data.next() {
                value.inner_html()
            } else {
                String::new()
            };
            (key.trim().into(), val)
        })
        .chain(vec![("name".into(), name)])
        .chain(vec![(
            "reg 28 compliant".into(),
            if is_reg_28_comliant {
                "true".into()
            } else {
                "false".into()
            },
        )])
        .collect::<std::collections::HashMap<_, _>>()
}
fn get_trusts(url: &str) -> Vec<UnitTrust> {
    let unit_trusts = get_links(&url);
    unit_trusts
        .iter()
        .take(1)
        .map(|link| {
            let url_query_fragment = link.split('?').collect::<Vec<_>>();
            let fund_details_url =
                url.to_owned() + "FundDetails.aspx?" + url_query_fragment[1];
            let document = get_document(&fund_details_url).unwrap();
            let reg_28_com_selector =
                scraper::Selector::parse("#FundHeader_Reg28").unwrap();
            let is_reg_28_comliant =
                document.select(&reg_28_com_selector).next().is_some();
            let info = get_detailed_information(&document, is_reg_28_comliant);
            let fees = get_fees_and_costs(&document);
            let statutory_info = get_statutory_data(&document);
            UnitTrust::from_hash_map(&info, &fees, &statutory_info)
        })
        .collect::<Vec<UnitTrust>>()
}

fn main() {
    let base_url = String::from("https://www.fundsdata.co.za/fdov2/");
    get_trusts(&base_url)
        .iter()
        .for_each(|trust| print!("{}", serde_json::to_string(trust).unwrap()))
}
