use std::fs::OpenOptions;
use std::io::Write;

pub fn init_logger() {
    env_logger::init();
}

pub fn write_log(message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/scraper.log")
        .expect("Failed to open log file");

    writeln!(file, "{}", message).expect("Failed to write log");
}
