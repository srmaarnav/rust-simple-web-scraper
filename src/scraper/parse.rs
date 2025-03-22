use scraper::Html;
use std::error::Error;
use crate::utils::logger::write_log;

pub fn parse_html(html: &str) -> Result<Html, Box<dyn Error>> {
    let document = Html::parse_document(html);
    write_log("HTML parsed successfully");
    Ok(document)
}
