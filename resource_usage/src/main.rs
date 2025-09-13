mod config_inject_periodic;
mod config_logger;
mod config_timescaledb;
mod messages;

use std::{
    net::{Ipv4Addr, SocketAddrV4},
    time::Duration,
};

use rsiot::{
    executor::{ComponentExecutor, ComponentExecutorConfig},
    logging::{LogConfig, LogConfigFilter},
};
use tokio::main;

use messages::*;

#[main]
async fn main() {
    LogConfig {
        filter: LogConfigFilter::String("info"),
        tokio_console_addr: SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 6669),
    }
    .run()
    .unwrap();

    let cmp_executor_config = ComponentExecutorConfig {
        buffer_size: 1000,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(200),
        fn_tokio_metrics: |metrics| Some(Msg::TokioRuntimeMetrics_(metrics)),
    };

    ComponentExecutor::<Msg>::new(cmp_executor_config)
        .add_cmp(config_inject_periodic::cmp1())
        // .add_cmp(cmp_inject_periodic::cmp1())
        // .add_cmp(cmp_inject_periodic::cmp1())
        // .add_cmp(cmp_inject_periodic::cmp1())
        .add_cmp(config_logger::cmp1())
        // .add_cmp(cmp_logger::cmp1())
        // .add_cmp(cmp_logger::cmp1())
        // .add_cmp(cmp_logger::cmp1())
        // .add_cmp(cmp_logger::cmp1())
        // .add_cmp(cmp_logger::cmp1())
        // .add_cmp(cmp_logger::cmp1())
        // .add_cmp(cmp_logger::cmp1())
        // .add_cmp(cmp_logger::cmp1())
        // .add_cmp(cmp_logger::cmp1())
        .add_cmp(config_timescaledb::cmp())
        .wait_result()
        .await
        .unwrap();
}
