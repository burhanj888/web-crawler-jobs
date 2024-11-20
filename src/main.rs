// src/main.rs

mod crawler;

#[tokio::main]
async fn main() {
    let url = "https://www.ycombinator.com/jobs"; // Replace with your target website URL

    match crawler::extract_company_names_from_url(url).await {
        Ok(company_names) => {
            for company in company_names {
                println!("Company Name: {}", company);
            }
        }
        Err(e) => eprintln!("Error extracting company names: {}", e),
    }
}
