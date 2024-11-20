use regex::Regex;
use reqwest;
use std::error::Error;

pub async fn extract_company_names_from_url(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // Fetch the HTML content from the URL
    let response = reqwest::get(url).await?.text().await?;

    // Create a regular expression to capture the company name
    let re = Regex::new(r#"companyName&quot;:&quot;([^&]+)&quot;"#)?;

    // Vector to hold extracted company names
    let mut company_names = Vec::new();

    // Iterate over all matches and extract the company name
    for cap in re.captures_iter(&response) {
        let company_name = &cap[1]; // Captured group 1
        company_names.push(company_name.to_string());
    }

    Ok(company_names)
}
