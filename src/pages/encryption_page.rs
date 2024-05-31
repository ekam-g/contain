use slint::slint;

slint! {
    import { Button, VerticalBox, ListView, ProgressIndicator, HorizontalBox } from "std-widgets.slint";


export component EncryptionPage inherits Window {
    preferred-width: 600px;
    preferred-height: 600px;
    in-out property <{path: string, time: int}> time_data;
    title: "Encrypt " + time_data.path + " - Contain";
    callback request-open-file();
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
                TextInput {
                    input-type : InputType.number;
                    text: 0;
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
                        //todo
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
                        root.request-open-file();
                    }
                }
            }
        }
    }
}

}

pub fn run() -> Result<(), slint::PlatformError> {
    let ui = EncryptionPage::new()?;
    //Todo improve error handing
    // let time_manger = Arc::new(Mutex::new(TimeManger::new().unwrap()));
    // let time_data: Rc<VecModel<(SharedString, i32)>> = Rc::new(VecModel::default()); 
    // time_manger.lock().unwrap().time_files.iter().for_each(|data| {
    //     time_data.push((data.path.clone().into(), data.time as i32))
    // });
    // ui.set_time_data(ModelRc::from(time_data));
    // ui.on_request_open_file({
    //     let ui_handle = ui.as_weak();   
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         let files = FileDialog::new()
    //             .add_filter("text", &["txt", "rs"])
    //             .add_filter("rust", &["rs", "toml"])
    //             .set_directory("/")
    //             .pick_file();
    //        }
    // });
    // ui.on_request_refresh({
    //     let time_manger = Arc::clone(&time_manger);
    //     move || {
    //         //todo finish this with https://releases.slint.dev/1.6.0/docs/rust/slint/fn.invoke_from_event_loop
            
    //         time_manger.lock().unwrap().update_time().unwrap();
    //        }
    // })
    ui.show()
}