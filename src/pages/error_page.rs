use std::
    sync::{atomic::{AtomicBool, Ordering}, Arc}
;

use slint::{slint, ComponentHandle};


slint! {
    import { Button, VerticalBox, ListView, ProgressIndicator, HorizontalBox } from "std-widgets.slint";


export component ErrorPage inherits Window {
    preferred-width: 320px;
    preferred-height: 180px;
    in property <string> prompt;
    in property <bool> show;
    title: "ERROR";
    callback yes();
    callback close();
    VerticalBox {
        Text {
            padding: 10px;
            text: prompt;
        }
        if show : HorizontalLayout {
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



pub fn run(error_text : String, show_buttion : bool) -> Result<bool, slint::PlatformError> {
    let ui = ErrorPage::new()?;
    let  input =    Arc::new(AtomicBool::new(false));
    ui.set_prompt(error_text.into());
    ui.set_show(show_buttion);
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
