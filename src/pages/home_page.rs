use rfd::FileDialog;
use slint::{ComponentHandle, Model};

use crate::time_manger::TimeManger;

slint::include_modules!();
pub fn run() -> Result<(), slint::PlatformError> {
    let ui = MyApp::new()?;
    //Todo improve error handing
    let time_manger = TimeManger::new().unwrap();
    let mut old_time_data = ui.get_time_data();
    time_manger.time_files.iter().for_each(time_file| {
        old_time_data.set_row_data(index, time_file.path.into());
    });
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