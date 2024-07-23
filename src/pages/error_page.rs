use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};

use slint::{slint, ComponentHandle};

use crate::time_manger::TimeManger;

slint! {
import { Button, VerticalBox, ListView, ProgressIndicator, HorizontalBox } from "std-widgets.slint";


export component ErrorPage inherits Window {
    preferred-width: 320px;
    preferred-height: 180px;
    in-out property <string> prompt;
    title: "ERROR";
    callback yes();
    VerticalBox {
        Text {
            text: prompt;
        }
        HorizontalLayout {
            padding: 10px;
            height: 12%;
            spacing: 15px;
            Rectangle {
                border-width: 1px;
                border-color: black;
                border-radius: 12px;
                Button {
                    width: 100%;
                    height: 100%;
                    text: "No";
                    clicked => {
                        root.close();
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
                    text: "Ok";
                    clicked => {
                        root.yes();
                        root.close();
                    }
                }
            }
        }
    }
}
}



pub fn run(path: PathBuf, time: &Arc<Mutex<TimeManger>>) -> Result<(), slint::PlatformError> {
    let ui = ErrorPage::new()?;
    thread::spawn(move || {});
    ui.on_close({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.hide().unwrap();
        }
    });
    ui.on_encrypt({
        let ui_handle = ui.as_weak();
        let time: Arc<Mutex<TimeManger>> = Arc::clone(&time);
        move || {
            let ui = ui_handle.unwrap();
            time.lock().unwrap().update_time().unwrap();
            time.lock()
                .unwrap()
                .add_file(path.clone(), ui.get_min().parse::<u128>().unwrap() * 60)
                .unwrap();
            ui.hide().unwrap();
        }
    });
    ui.show()
}
