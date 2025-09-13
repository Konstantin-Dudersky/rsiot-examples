use std::time::Duration;

use rsiot::{components::cmp_timescaledb::*, executor::Component};

use crate::messages::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        connection_string: "postgres://postgres:postgres@localhost:5432/db_data".into(),
        max_connections: 5,
        send_period: Duration::from_secs(2),
        fn_input: |msg| {
            let rows = match msg {
                Msg::TokioRuntimeMetrics_(msg) => msg.into(),
                _ => return None,
            };
            Some(rows)
        },
    };

    Cmp::new(config)
}
