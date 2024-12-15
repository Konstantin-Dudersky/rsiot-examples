use std::time::Duration;

use rsiot::{
    components::{
        cmp_inject_periodic,
        cmp_linux_uart_master::{self, devices::test_device},
        cmp_logger,
    },
    executor::{ComponentExecutor, ComponentExecutorConfig},
    message::{example_service::Service, Message},
};
use tracing::Level;

use messages::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(data) = msg.get_custom_data() else {
                return Ok(None);
            };
            match data {
                Custom::CounterEsp(_) => Ok(Some(msg.serialize_data()?)),
                _ => Ok(None),
            }
        },
    };

    // cmp_inject_periodic -------------------------------------------------------------------------
    let mut counter: u32 = 0;
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_millis(1000),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::CounterRpi(counter));
            counter += 1;
            vec![msg]
        },
    };

    // cmp_linux_uart ------------------------------------------------------------------------------
    let config_linux_uart = cmp_linux_uart_master::Config {
        // port: "/dev/ttyAMA0",
        port: "/dev/ttyUSB0",
        baudrate: cmp_linux_uart_master::Baudrate::_115_200,
        data_bits: cmp_linux_uart_master::DataBits::_8,
        stop_bits: cmp_linux_uart_master::StopBits::_1,
        parity: cmp_linux_uart_master::Parity::None,
        wait_after_write: Duration::from_millis(50),
        gpio_chip: "/dev/gpiochip0",
        pin_rts: None,
        devices: vec![Box::new(test_device::TestDevice {
            address: 1,
            fn_input: |msg, buffer| {
                let Some(msg) = msg.get_custom_data() else {
                    return;
                };
                if let Custom::CounterRpi(counter) = msg {
                    buffer.counter_rpi = counter
                }
            },
            fn_output: |buffer| {
                let msg = Message::new_custom(Custom::CounterEsp(buffer.counter_esp));
                vec![msg]
            },
        })],
    };

    // executor ------------------------------------------------------------------------------------
    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        delay_publish: Duration::from_millis(100),
        fn_auth: |msg, _| Some(msg),
    };

    ComponentExecutor::<Custom>::new(executor_config)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_linux_uart_master::Cmp::new(config_linux_uart))
        .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
        .wait_result()
        .await
        .unwrap();
}
