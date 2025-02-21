use reactive_stores::Store;
use rsiot::components::{cmp_leptos::StoreBound, cmp_plc::plc::library::drives};

#[derive(Clone, Default, Store)]
pub struct InputStore {
    pub m1_status: drives::motor::QHmiStatus,
}

impl StoreBound for InputStore {}

#[derive(Clone, Default, Store)]
pub struct OutputStore {
    pub m1_command: drives::motor::IHmiCommand,
    pub m2_command: drives::motor::IHmiCommand,
}

impl StoreBound for OutputStore {}
