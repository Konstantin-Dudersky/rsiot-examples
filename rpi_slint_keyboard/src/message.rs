use rsiot::message::{example_service::Service, MsgDataBound};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    GpioTab(bool),
    GpioNumber(bool),
    GpioEnter(bool),
    GpioBackspace(bool),
    Weight(f64),
}

impl MsgDataBound for Custom {
    type TService = Service;
}
