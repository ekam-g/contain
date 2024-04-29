use std::{rc::Rc, sync::{Arc, Mutex}, thread};

use rfd::FileDialog;
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

use crate::time_manger::{self, time_file, TimeManger};

slint::include_modules!();
pub fn run() -> Result<(), slint::PlatformError> {
    let ui = MyApp::new()?;
    //Todo improve error handing
    let time_manger = Arc::new(Mutex::new(TimeManger::new().unwrap()));
    let time_data: Rc<VecModel<(SharedString, i32)>> = Rc::new(VecModel::default()); 
    time_manger.lock().unwrap().time_files.iter().for_each(|data| {
        time_data.push((data.path.clone().into(), data.time as i32))
    });
    ui.set_time_data(ModelRc::from(time_data));
    ui.on_request_open_file({
        let ui_handle = ui.as_weak();   
        move || {
            let ui = ui_handle.unwrap();
            let files = FileDialog::new()
                .add_filter("text", &["txt", "rs"])
                .add_filter("rust", &["rs", "toml"])
                .set_directory("/")
                .pick_file();
           }
    });
    ui.on_request_refresh({
        let time_manger = Arc::clone(&time_manger);
        move || {
            //todo finish this
            time_manger.lock().unwrap().update_time().unwrap();
           }
    });
    ui.run()
}