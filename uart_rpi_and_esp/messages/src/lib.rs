use rsiot::message::{MsgDataBound, MsgKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Custom {
    CounterEsp(u32),
    CounterRpi(u32),
}

impl MsgDataBound for Custom {}

#[derive(Debug, Clone, PartialEq)]
pub enum Services {
    Esp,
    Rpi,
}
