use std::{
    path::PathBuf, sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, thread
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
    callback close();
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



pub fn run(error_text : String) -> Result<bool, slint::PlatformError> {
    let ui = ErrorPage::new()?;
    let  input =    Arc::new(AtomicBool::new(false));
    ui.set_prompt(error_text.into());
    ui.on_close({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.hide().unwrap();
        }
    });
    ui.on_yes({
        let input = Arc::clone(&input);
        move || {
            input.store(true, Ordering::Release);
        }
    });
    ui.show()?;
    Ok(input.load(Ordering::Relaxed))
}
