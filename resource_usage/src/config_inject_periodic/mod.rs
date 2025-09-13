use std::time::Duration;

use rsiot::components::cmp_inject_periodic::*;

use crate::messages::*;

pub fn cmp1() -> rsiot::executor::Component<Config<Msg, impl FnMut() -> Vec<Msg>>, Msg> {
    let mut counter = 0;

    let config = Config {
        period: Duration::from_millis(1),
        fn_periodic: move || {
            let data = Data::new(counter as f64);
            let msg = Msg::ServerCounter(data);
            counter += 1;
            if counter >= 100 {
                counter = 0;
            }
            vec![msg]
        },
    };

    Cmp::new(config)
}
