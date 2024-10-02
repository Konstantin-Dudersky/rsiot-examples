use rsiot::message::{example_service::Service, MsgDataBound};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    GpioTab(bool),
    Gpio1(bool),
    Gpio2(bool),
    GpioBackspace(bool),
}

impl MsgDataBound for Custom {
    type TService = Service;
}
