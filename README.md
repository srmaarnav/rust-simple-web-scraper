# Rust Web Scrapper

A simple Rust application that fetches a webpage and extracts titles using the `reqwest` and `scraper` crates. It logs operations to `logs/scraper.log` and prints scraped titles to the console.

## Getting Started

1. **Build the project:**

   ```sh
   cargo build
   ```

2. **Run the application:**

   ```sh
   cargo run
   ```

## Overview

- The main entry point is in main.rs.
- HTTP fetching is implemented in fetch.rs.
- HTML parsing and title extraction are handled in parse.rs and extract.rs respectively.
- Logging is set up in logger.rs.
