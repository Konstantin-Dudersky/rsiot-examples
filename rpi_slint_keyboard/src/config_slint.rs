use std::sync::Arc;

use rsiot::components::cmp_slint;
use slint::{include_modules, platform::WindowEvent, ComponentHandle, SharedString, Weak};
use tokio::sync::Mutex;

use crate::{Custom, MainWindow};

pub fn config(slint_inst: Weak<MainWindow>) -> cmp_slint::Config<MainWindow, Custom> {
    cmp_slint::Config {
        instance: Arc::new(Mutex::new(slint_inst)),
        fn_input: |msg, window| {
            let Some(msg) = msg.get_custom_data() else {
                return;
            };
            match msg {
                Custom::GpioTab(value) => {
                    if !value {
                        return;
                    }
                    let key: SharedString = slint::platform::Key::Tab.into();
                    window
                        .upgrade_in_event_loop(move |h| {
                            h.window()
                                .dispatch_event(WindowEvent::KeyPressed { text: key.clone() });
                            h.window()
                                .dispatch_event(WindowEvent::KeyReleased { text: key })
                        })
                        .unwrap()
                }
                Custom::Gpio1(value) => {
                    if !value {
                        return;
                    }
                    let key: SharedString = SharedString::from("1");
                    window
                        .upgrade_in_event_loop(move |h| {
                            h.window()
                                .dispatch_event(WindowEvent::KeyPressed { text: key.clone() });
                            h.window()
                                .dispatch_event(WindowEvent::KeyReleased { text: key })
                        })
                        .unwrap()
                }
                Custom::Gpio2(value) => {
                    if !value {
                        return;
                    }
                    let key: SharedString = SharedString::from("2");
                    window
                        .upgrade_in_event_loop(move |h| {
                            h.window()
                                .dispatch_event(WindowEvent::KeyPressed { text: key.clone() });
                            h.window()
                                .dispatch_event(WindowEvent::KeyReleased { text: key })
                        })
                        .unwrap()
                }
                Custom::GpioBackspace(value) => {
                    if !value {
                        return;
                    }
                    let key: SharedString = slint::platform::Key::Backspace.into();
                    window
                        .upgrade_in_event_loop(move |h| {
                            h.window()
                                .dispatch_event(WindowEvent::KeyPressed { text: key.clone() });
                            h.window()
                                .dispatch_event(WindowEvent::KeyReleased { text: key })
                        })
                        .unwrap()
                }
            }
        },
        fn_output: |_window, _tx| {},
    }
}
