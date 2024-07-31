// #![feature(const_for)]
pub mod encryption;
pub mod pages;
pub mod time_manger;
use std::process::exit;

use pages::home_page;

pub const TEST_VALUE: &str = "This works very well";

#[tokio::main]
async fn main() {
    home_page::run().await.expect("Failed to Start App");
    exit(0);
}
