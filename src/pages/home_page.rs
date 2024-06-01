use slint::slint;
use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

use rfd::FileDialog;
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

use crate::time_manger::{self, time_file, TimeManger};

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
                        text: data.path;
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
    let time_data: Rc<VecModel<(SharedString, i32)>> = Rc::new(VecModel::default());
    time_manger
        .lock()
        .unwrap()
        .time_files
        .iter()
        .for_each(|data| time_data.push((data.path.clone().into(), data.time as i32)));
    ui.set_time_data(ModelRc::from(time_data));
    ui.on_request_open_file({
        let ui_handle = ui.as_weak();
        let time_manger: Arc<Mutex<TimeManger>> = Arc::clone(&time_manger);
        move || {
            let ui = ui_handle.unwrap();
            let file = FileDialog::new().pick_file();
            if let Some(file_checked) = file {
                encryption_page::run(file_checked, &time_manger).unwrap();
            }
        }
    });
    ui.on_request_refresh({
        let time_manger: Arc<Mutex<TimeManger>> = Arc::clone(&time_manger);
        move || {
            //todo finish this with https://releases.slint.dev/1.6.0/docs/rust/slint/fn.invoke_from_event_loop
            let mut time = time_manger.lock().unwrap();
            time.update_time().unwrap();
            time.decrypt_old_files().unwrap();
        }
    });
    ui.run()
}
