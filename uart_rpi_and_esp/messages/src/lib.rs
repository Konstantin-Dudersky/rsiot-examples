use rsiot::message::{MsgDataBound, ServiceBound};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    CounterEsp(u32),
    CounterRpi(u32),
}

impl MsgDataBound for Custom {
    type TService = Services;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Services {
    Esp,
    Rpi,
}

impl ServiceBound for Services {}
