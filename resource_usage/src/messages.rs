use rsiot::{
    executor::TokioRuntimeMetrics,
    message::{MsgDataBound, MsgKey},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    ServerCounter(Data),
    Empty0,
    Empty1,
    Empty2,
    Empty3,
    Empty4,
    Empty5,
    Empty6,
    Empty7,
    Empty8,
    Empty9,
    Empty10,
    TokioRuntimeMetrics_(TokioRuntimeMetrics),
}
impl MsgDataBound for Msg {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Data {
    pub counter_0: f64,
    pub counter_1: f64,
    pub counter_2: f64,
    pub counter_3: f64,
    pub counter_4: f64,
    pub counter_5: f64,
    pub counter_6: f64,
    pub counter_7: f64,
    pub counter_8: f64,
    pub counter_9: f64,
    pub counter_10: f64,
    pub counter_11: f64,
    pub counter_12: f64,
    pub counter_13: f64,
    pub counter_14: f64,
    pub counter_15: f64,
    pub counter_16: f64,
    pub counter_17: f64,
    pub counter_18: f64,
    pub counter_19: f64,
}
impl Data {
    pub fn new(counter: f64) -> Self {
        Self {
            counter_0: counter,
            counter_1: counter,
            counter_2: counter,
            counter_3: counter,
            counter_4: counter,
            counter_5: counter,
            counter_6: counter,
            counter_7: counter,
            counter_8: counter,
            counter_9: counter,
            counter_10: counter,
            counter_11: counter,
            counter_12: counter,
            counter_13: counter,
            counter_14: counter,
            counter_15: counter,
            counter_16: counter,
            counter_17: counter,
            counter_18: counter,
            counter_19: counter,
        }
    }
}
