mod config_inject_periodic;
mod config_slint;
mod config_svg;
mod messages;

use rsiot::executor::{ComponentExecutor, ComponentExecutorConfig};
use slint::{ComponentHandle, Weak};
use std::time::Duration;
use tokio::main;

use messages::*;

fn main() -> anyhow::Result<()> {
    let main_window = config_slint::MainWindow::new()?;
    let main_window_link = main_window.as_weak();
    std::thread::spawn(move || main_executor(main_window_link));
    main_window.run()?;
    Err(anyhow::Error::msg("Program stop"))
}

#[main]
async fn main_executor(slint_window: Weak<config_slint::MainWindow>) -> anyhow::Result<()> {
    let executor_config = ComponentExecutorConfig::<Msg> {
        buffer_size: 1000,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };
    ComponentExecutor::new(executor_config)
        .add_cmp(config_slint::cmp(slint_window))
        .add_cmp(config_inject_periodic::cmp())
        .add_cmp(config_svg::cmp())
        .wait_result()
        .await?;
    Err(anyhow::Error::msg("Program stop"))
}
