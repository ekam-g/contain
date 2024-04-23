// #![feature(const_for)]
pub mod time_manger;
pub mod encryption;
pub mod pages;
use pages::home_page;
use slint::ComponentHandle;

slint::include_modules!();
fn main() -> Result<(), slint::PlatformError> {
    home_page::run()
}