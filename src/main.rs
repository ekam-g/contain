// #![feature(const_for)]
pub mod encryption;
pub mod pages;
pub mod time_manger;
use pages::home_page;
use slint::ComponentHandle;

fn main() -> Result<(), slint::PlatformError> {
    home_page::run()
}
