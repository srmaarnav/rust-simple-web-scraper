mod scraper;
mod utils;

use scraper::fetch::fetch_page;
use scraper::parse::parse_html;
use scraper::extract::extract_titles;
use utils::logger::init_logger;

fn main() {
    init_logger();
    let url = "https://news.ycombinator.com/";

    match fetch_page(url) {
        Ok(html) => {
            match parse_html(&html) {
                Ok(document) => {
                let selector = "span.titleline > a"; // Updated selector
                    match extract_titles(&document, selector) {
                        Ok(titles) => {
                            for title in titles {
                                println!("{}", title);
                            }
                        }
                        Err(e) => eprintln!("Error extracting titles: {}", e),
                    }
                }
                Err(e) => eprintln!("Error parsing HTML: {}", e),
            }
        }
        Err(e) => eprintln!("Error fetching page: {}", e),
    }
}
