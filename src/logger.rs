use chrono::Local;
use log::debug;

pub fn print_info(msg: &str) {
    println!("[{}] {}", log_prefix(), msg);
}

pub fn print_error(msg: &str) {
    eprintln!("[{}] {}", log_prefix(), msg);
}

pub fn print_debug(msg: &str) {
    debug!("[{}] {}", log_prefix(), msg);
}

fn log_prefix() -> impl std::fmt::Display {
    Local::now().format("%H:%M:%S")
}
