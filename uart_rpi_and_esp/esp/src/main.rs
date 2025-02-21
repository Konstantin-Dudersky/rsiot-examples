use std::time::Duration;

use esp_idf_svc::hal::prelude::Peripherals;
use rsiot::{
    components::{
        cmp_esp_uart_slave::{self},
        cmp_inject_periodic, cmp_logger,
    },
    components_config::uart_general::{UartRequest, UartResponse},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    logging::configure_logging,
    message::Message,
};
use serde::{Deserialize, Serialize};
use tokio::task::LocalSet;
use tracing::{info, level_filters::LevelFilter, Level};

use messages::{Custom, Services};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    esp_idf_svc::sys::link_patches();

    configure_logging(LevelFilter::INFO).unwrap();

    let peripherals = Peripherals::take().unwrap();

    // cmp_logger ----------------------------------------------------------------------------------
    let config_logger = cmp_logger::Config::<Custom> {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(data) = msg.get_custom_data() else {
                return Ok(None);
            };
            if let Custom::CounterRpi(_) = data {
                Ok(Some(msg.serialize_data()?))
            } else {
                Ok(None)
            }
        },
    };

    // cmp_esp_uart_slave --------------------------------------------------------------------------
    let config_esp_uart_slave = cmp_esp_uart_slave::Config::<_, _, _, _> {
        address: 1,
        uart: peripherals.uart1,
        pin_rx: peripherals.pins.gpio20.into(),
        pin_tx: peripherals.pins.gpio21.into(),
        pin_rts: peripherals.pins.gpio4.into(),
        baudrate: cmp_esp_uart_slave::Baudrate::_115_200,
        data_bits: cmp_esp_uart_slave::DataBits::_8,
        parity: cmp_esp_uart_slave::Parity::None,
        stop_bits: cmp_esp_uart_slave::StopBits::_1,
        buffer_data_default: BufferData::default(),
        fn_input: |msg: &Message<Custom>, buffer: &mut BufferData| {
            let Some(msg) = msg.get_custom_data() else {
                return;
            };
            if let Custom::CounterEsp(counter) = msg {
                buffer.counter_esp = counter;
            }
        },
        fn_uart_comm: |mut fieldbus_request: UartRequest, buffer: &mut BufferData| {
            let request: Request = fieldbus_request.get_payload()?;

            info!("Request: {:?}", request);

            let response = match request {
                Request::GetCounterFromEsp => Response::CounterFromEsp(buffer.counter_esp),
                Request::SetCounter(counter_rpi) => {
                    buffer.counter_rpi = counter_rpi;
                    Response::Ok
                }
            };
            let response = UartResponse::new(response);

            Ok(response)
        },
        fn_output: |buffer| {
            let msg = Message::new_custom(Custom::CounterRpi(buffer.counter_rpi));
            vec![msg]
        },
        fn_output_period: Duration::from_millis(100),
    };

    // cmp_inject_periodic -------------------------------------------------------------------------
    let mut counter: u32 = 0;
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_millis(500),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::CounterEsp(counter));
            counter += 1;
            vec![msg]
        },
    };

    // executor ------------------------------------------------------------------------------------
    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Services::Esp,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Custom, Services>::new(executor_config)
            .add_cmp(cmp_logger::Cmp::new(config_logger))
            .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
            .add_cmp(cmp_esp_uart_slave::Cmp::new(config_esp_uart_slave))
            .wait_result()
            .await
            .unwrap()
    });

    local_set.await;
}

#[derive(Default)]
struct BufferData {
    pub counter_esp: u32,
    pub counter_rpi: u32,
}

/// Запросы
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Request {
    /// Запрос значения счетчика из ESP32
    GetCounterFromEsp,
    /// Передать значение своего счетчика
    SetCounter(u32),
}

/// Ответы
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Response {
    /// Счетчик из ESP32
    CounterFromEsp(u32),
    /// Ok
    Ok,
}

/// Буфер данных
#[derive(Default)]
pub struct Buffer {}
