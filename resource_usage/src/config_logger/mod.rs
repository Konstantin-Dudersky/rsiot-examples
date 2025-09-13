use rsiot::{components::cmp_logger::*, executor::Component};

use crate::messages::*;

pub fn cmp1() -> Component<Config<Msg>, Msg> {
    let config = Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };

            let text = match msg {
                // Msg::ServerCounter(data) => format!("Counter: {:?}", data.counter_0),
                // Msg::TokioRuntimeMetrics_(metrics) => format!("Tokio Metrics: {metrics:#?}"),
                _ => return Ok(None),
            };
            // Ok(None)
            Ok(Some(text))
        },
    };

    Cmp::new(config)
}
