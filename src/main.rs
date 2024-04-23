// #![feature(const_for)]
pub mod time_manger;
pub mod encryption;
use rfd::FileDialog;
use slint::ComponentHandle;

slint::include_modules!();
fn main() -> Result<(), slint::PlatformError> {
    let ui = MyApp::new()?;
    ui.on_request_open_file({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let files = FileDialog::new()
                .add_filter("text", &["txt", "rs"])
                .add_filter("rust", &["rs", "toml"])
                .set_directory("/")
                .pick_file();
            ui.set_info(files.unwrap().as_path().to_str().unwrap().into());
           }
    });
    ui.run()
}