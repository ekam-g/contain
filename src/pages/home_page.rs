use futures::executor::block_on;
use slint::{slint, Weak};
use std::{rc::Rc, sync::Arc, thread, time::Duration};
use tokio::sync::Mutex;

use rfd::FileDialog;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

use crate::time_manger::TimeManger;

use super::{encryption_page, error_page};
slint! {
    import { Button, VerticalBox, ListView, ProgressIndicator } from "std-widgets.slint";


export component MyApp inherits Window {
    preferred-width: 600px;
    preferred-height: 600px;
    title: "Home - Contain";
    in-out property <[{path: string, time: int}]> time_data;
    callback request-open-file();
    callback request-open-folder();
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
                    text: "Add File";
                    clicked => {
                        root.request-open-file();
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
                    text: "Add Folder";
                    clicked => {
                        root.request-open-folder();
                    }
                }
            }
        }
    }
}

}
pub async fn run() -> Result<(), slint::PlatformError> {
    let ui = MyApp::new()?;
    let time_manger = Arc::new(Mutex::new(TimeManger::new().await.unwrap()));
    remove_old_files_thread(Arc::clone(&time_manger), ui.as_weak()).await;
    ui.set_time_data(ModelRc::from(block_on(update_time_data(&time_manger))));
    let time: Arc<Mutex<TimeManger>> = Arc::clone(&time_manger);
    ui.on_request_open_file({
        move || {
            let time_manger: Arc<Mutex<TimeManger>> = Arc::clone(&time_manger);
            tokio::spawn(async move {
                // No Wifi
                if time_manger.lock().await.get_time().is_none() {
                    slint::invoke_from_event_loop(move || {
                        error_page::run(
                            "Time is unknown(connection to api failed)".to_owned(),
                            false,
                        )
                        .unwrap();
                    })
                    .unwrap();
                    return;
                }
                let file = FileDialog::new().pick_file();
                if let Some(file_checked) = file {
                    slint::invoke_from_event_loop(move || {
                        encryption_page::run(file_checked, &time_manger).unwrap()
                    })
                    .unwrap();
                }
            });
        }
    });
    ui.on_request_open_folder({
        move || {
            let time_manger: Arc<Mutex<TimeManger>> = Arc::clone(&time);
            tokio::spawn(async move {
                // No Wifi
                if time_manger.lock().await.get_time().is_none() {
                    slint::invoke_from_event_loop(move || {
                        error_page::run(
                            "Time is unknown(connection to api failed)".to_owned(),
                            false,
                        )
                        .unwrap();
                    })
                    .unwrap();
                    return;
                }
                let file = FileDialog::new().pick_folder();
                if let Some(file_checked) = file {
                    slint::invoke_from_event_loop(move || {
                        encryption_page::run(file_checked, &time_manger).unwrap()
                    })
                    .unwrap();
                }
            });
        }
    });
    ui.run()
}

async fn update_time_data(
    time_manger: &Arc<Mutex<TimeManger>>,
) -> Rc<VecModel<(SharedString, i32)>> {
    let time_data: Rc<VecModel<(SharedString, i32)>> = Rc::new(VecModel::default());
    let time = time_manger.lock().await;
    let actual_time  = time.get_time().unwrap_or_default();
    time.time_files.iter().for_each(|data| {
        time_data.push((
            data.path.clone().into(),
            ((data.time - actual_time) as i32 / 60) + 1,
        ))
    });
    time_data
}

async fn remove_old_files_thread(time_manger: Arc<Mutex<TimeManger>>, ui_handle: Weak<MyApp>) {
    tokio::task::spawn_blocking(move || {
        block_on(async {
            loop {
                thread::sleep(Duration::from_millis(500));
                match time_manger.lock().await.decrypt_old_files().await {
                    Err(e) => {
                        error_page::run(format!("Failed To Decrypt Due To: {}", e), false).unwrap();
                        slint::invoke_from_event_loop(move || {
                            error_page::run(format!("Failed To Decrypt Due To: {}", e), false)
                                .unwrap();
                        })
                        .unwrap();
                        continue;
                    }
                    Ok(failed) => {
                        for (path, e) in failed {
                            let time_manger: Arc<Mutex<TimeManger>> = Arc::clone(&time_manger);
                            slint::invoke_from_event_loop(move || {
                                let mut time = block_on(time_manger.lock());
                                let input: bool = error_page::run(
                                    format!("Failed To Decrypt {} Due To: {}", path, e),
                                    true,
                                )
                                .unwrap();
                                if input {
                                    time.time_files = time
                                        .time_files
                                        .clone()
                                        .into_iter()
                                        .filter(|time_file| time_file.path != *path)
                                        .collect();
                                }
                            })
                            .unwrap();
                        }
                        let time_manger: Arc<Mutex<TimeManger>> = Arc::clone(&time_manger);
                        ui_handle
                            .upgrade_in_event_loop(move |handle| {
                                let data = block_on(update_time_data(&time_manger));
                                handle.set_time_data(ModelRc::from(data));
                            })
                            .unwrap();
                    }
                }
            }
        })
    });
}
