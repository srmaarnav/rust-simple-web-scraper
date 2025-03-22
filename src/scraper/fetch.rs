use reqwest;
use std::error::Error;
use crate::utils::logger::write_log;

pub fn fetch_page(url: &str) -> Result<String, Box<dyn Error>> {
    match reqwest::blocking::get(url) {
        Ok(response) => {
            let body = response.text()?;
            write_log(&format!("Fetched URL: {}", url));
            Ok(body)
        }
        Err(e) => {
            write_log(&format!("Failed to fetch {}: {}", url, e));
            Err(Box::new(e))
        }
    }
}
