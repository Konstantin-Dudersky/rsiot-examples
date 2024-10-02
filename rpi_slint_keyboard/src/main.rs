mod config_raspberrypi_gpio;
// mod config_slint;
mod message;

use std::{sync::Arc, time::Duration};

use rsiot::{
    components::{cmp_raspberrypi_gpio, cmp_slint},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    message::example_service::Service,
};
use slint::{include_modules, platform::WindowEvent, SharedString, Weak};
use tokio::sync::Mutex;
use tracing::{info, Level};

use message::*;

include_modules!();

fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let main_window = MainWindow::new().unwrap();

    let main_window_link = main_window.as_weak();
    std::thread::spawn(move || main_executor(main_window_link));
    main_window.run().unwrap();
}

#[tokio::main]
async fn main_executor(slint_inst: Weak<MainWindow>) {
    // cmp_raspberrypi_gpio ------------------------------------------------------------------------
    let config_raspberrypi_gpio = config_raspberrypi_gpio::config();

    // cmp_slint -----------------------------------------------------------------------------------
    let config_slint = cmp_slint::Config {
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
    };

    // executor ------------------------------------------------------------------------------------
    let executor_config = ComponentExecutorConfig {
        buffer_size: 1000,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    ComponentExecutor::<Custom>::new(executor_config)
        .add_cmp(cmp_slint::Cmp::new(config_slint))
        // .add_cmp(cmp_raspberrypi_gpio::Cmp::new(config_raspberrypi_gpio))
        .wait_result()
        .await
        .unwrap();
}
