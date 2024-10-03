use rsiot::{components::cmp_raspberrypi_gpio, message::Message};

use crate::message::Custom;

pub fn config() -> cmp_raspberrypi_gpio::Config<Custom> {
    cmp_raspberrypi_gpio::Config {
        inputs: vec![
            cmp_raspberrypi_gpio::ConfigInput {
                pin_number: 17,
                fn_output: |value| Message::new_custom(Custom::GpioEnter(value)),
                pull_mode: cmp_raspberrypi_gpio::PullMode::Down,
            },
            cmp_raspberrypi_gpio::ConfigInput {
                pin_number: 23,
                fn_output: |value| Message::new_custom(Custom::GpioNumber(value)),
                pull_mode: cmp_raspberrypi_gpio::PullMode::Down,
            },
            cmp_raspberrypi_gpio::ConfigInput {
                pin_number: 22,
                fn_output: |value| Message::new_custom(Custom::GpioBackspace(value)),
                pull_mode: cmp_raspberrypi_gpio::PullMode::Down,
            },
            cmp_raspberrypi_gpio::ConfigInput {
                pin_number: 27,
                fn_output: |value| Message::new_custom(Custom::GpioTab(value)),
                pull_mode: cmp_raspberrypi_gpio::PullMode::Down,
            },
        ],
        outputs: vec![],
    }
}
