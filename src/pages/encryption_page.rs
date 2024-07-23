use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use slint::{slint, ComponentHandle};

use crate::time_manger::TimeManger;

slint! {
    import { Button, VerticalBox, ListView, ProgressIndicator, HorizontalBox } from "std-widgets.slint";


export component EncryptionPage inherits Window {
    preferred-width: 250px;
    preferred-height: 100px;
    in-out property <{path: string, time: int}> time_data;
    in-out property <string> min;
    title: time_data.path;
    callback close();
    callback encrypt();
    VerticalBox {
        Rectangle {
            border-width: 1px;
            border-color: black;
            border-radius: 12px;
            height: 10%;
            width: 80%;
            HorizontalBox {
                Text {
                    text: "Minutes";
                }
                input := TextInput {
                    input-type : InputType.number;
                    text: 0;
                    edited => {
                        min = self.text
                    }
                }
            }
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
                    text: "Cancel";
                    clicked => {
                        root.close()
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
                        root.encrypt();
                        root.close();
                    }
                }
            }
        }
    }
}

}
//todo make the error  page
pub fn run(path: PathBuf, time: &Arc<Mutex<TimeManger>>) -> Result<(), slint::PlatformError> {
    let ui = EncryptionPage::new()?;
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
