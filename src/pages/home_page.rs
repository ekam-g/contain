use slint::{slint, Weak};
use std::{
    rc::Rc, sync::{Arc, Mutex}, thread, time::Duration
};

use rfd::FileDialog;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

use crate::time_manger::TimeManger;

use super::encryption_page;
slint! {
    import { Button, VerticalBox, ListView, ProgressIndicator } from "std-widgets.slint";


export component MyApp inherits Window {
    preferred-width: 600px;
    preferred-height: 600px;
    title: "Home - Contain";
    in-out property <[{path: string, time: int}]> time_data;
    callback request-open-file();
    callback request-refresh();
    VerticalBox {
        Rectangle {
            height: 78%;
            if time_data.length == 0:
            ProgressIndicator {
                indeterminate: true;
                width: 80%;
                height: 20px;
            }
            if time_data.length != 0:
        ListView {
                for data in time_data: Rectangle {
                    height: 50px;
                    width: parent.width;
                    Text {
                        text: data.path + " " +  data.time;

                    }
                }
            }
        }

        HorizontalLayout {
            padding: 10px;
            height: 12%;
            spacing: 10px;
            Rectangle {
                border-width: 1px;
                border-color: black;
                border-radius: 12px;
                Button {
                    width: 100%;
                    height: 100%;
                    text: "Time Refresh";
                    clicked => {
                        root.request-refresh();
                    }
                }
            }

            Rectangle {
                border-width: 1px;
                border-color: black;
                border-radius: 12px;
                Button {
                    width: 100%;
                    height: 100%;
                    text: "Get File";
                    clicked => {
                        root.request-open-file();
                    }
                }
            }
        }
    }
}

}
pub fn run() -> Result<(), slint::PlatformError> {
    let ui = MyApp::new()?;
    //Todo improve error handing
    let time_manger = Arc::new(Mutex::new(TimeManger::new().unwrap()));
    ui.set_time_data(ModelRc::from(update_time_data(time_manger.clone())));
    ui.on_request_open_file({
        // let ui_handle: slint::Weak<MyApp> = ui.as_weak();
        let time_manger: Arc<Mutex<TimeManger>> = Arc::clone(&time_manger);
        move || {
            let file = FileDialog::new().pick_file();
            if let Some(file_checked) = file {
                encryption_page::run(file_checked, &time_manger).unwrap();
            }
        }
    });
    ui.on_request_refresh({
        let time_manger: Arc<Mutex<TimeManger>> = Arc::clone(&time_manger);
        let ui_handle = ui.as_weak();
        move || {
            let time_manger: Arc<Mutex<TimeManger>> = Arc::clone(&time_manger);
            let ui_handle = ui_handle.clone();
            // remove_old_files(time_manger, ui_handle)
        }
    });
    ui.run()
}

fn update_time_data(time_manger: Arc<Mutex<TimeManger>>) -> Rc<VecModel<(SharedString, i32)>> {
    let time_data: Rc<VecModel<(SharedString, i32)>> = Rc::new(VecModel::default());
    let time = time_manger.lock().unwrap();
    time.time_files.iter().for_each(|data| {
        time_data.push((
            data.path.clone().into(),
            ((data.time - time.current_unix_time.unwrap_or_default()) as i32 / 60) + 1,
        ))
    });
    time_data
}
fn remove_old_files_thread(time_manger: Arc<Mutex<TimeManger>>, ui_handle: Weak<MyApp>) {
    std::thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(500));
        let mut time = time_manger.lock().unwrap();
        if time.update_time().is_err() {
            continue;
        }
        if time.decrypt_old_files().is_err() {
            //todo handle this error better
            println!("Fail To Decrypt File");
        }
        let time_manger: Arc<Mutex<TimeManger>> = Arc::clone(&time_manger);
        ui_handle
            .upgrade_in_event_loop(move |handle| {
                let data = update_time_data(time_manger.clone());
                handle.set_time_data(ModelRc::from(data));
            })
            .unwrap();
    });
}
