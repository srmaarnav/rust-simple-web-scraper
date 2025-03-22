use scraper::{Html, Selector};
use std::error::Error;
use crate::utils::logger::write_log;

pub fn extract_titles(document: &Html, selector: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let selector = Selector::parse(selector).unwrap();
    let mut titles = Vec::new();

    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");
        println!("{}", text); // Debugging print statement
        titles.push(text.clone()); // Clone before moving
    }

    write_log("Extracted titles successfully");
    Ok(titles)
}
