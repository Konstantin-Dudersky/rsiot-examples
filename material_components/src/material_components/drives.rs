use leptos::prelude::*;
use reactive_stores::Store;
use rsiot::components::cmp_plc::plc::library::drives as plc_drives;
use rsiot::{
    components::cmp_leptos::{
        components::{
            plc_lib::drives,
            svg_dynamic::{SvgDynamic, SvgInput, SvgOutput},
        },
        create_signal_from_msg,
    },
    message::*,
};

use crate::{messages::*, stores::*};

#[component]
pub fn Drives() -> impl IntoView {
    let input_store = expect_context::<Store<InputStore>>();
    let output_store = expect_context::<Store<OutputStore>>();

    let motor_status = Signal::derive(move || input_store.m1_status().get());

    let (motor_command, motor_command_set) = signal(plc_drives::motor::IHmiCommand::default());
    Effect::new(move || {
        let motor_command = motor_command.get();
        output_store.m1_command().set(motor_command);
    });

    let (valve_analog_status, _) = create_signal_from_msg!("Custom-valve_analog_status");
    let (_, valve_analog_command) = create_signal_from_msg!("Custom-valve_analog_command");

    let (valve_status, _) = create_signal_from_msg!("Custom-valve_hmi_status");
    let (_, valve_command) = create_signal_from_msg!("Custom-valve_hmi_command");

    let (fpt, fpt_set) = signal(String::from(""));

    let svg_file = include_str!("../../schemas/drives.svg");
    let svg_input = vec![
        SvgInput::plc_drives_motor("motor", motor_status),
        SvgInput::plc_drives_valve_analog("valve_analog", valve_analog_status),
        SvgInput::plc_drives_valve("valve", valve_status),
    ];
    let svg_output = SvgOutput {
        ids: ["motor", "valve_analog", "valve"]
            .iter()
            .map(|id| id.to_string())
            .collect(),
        callback: move |id| match id {
            "motor" => fpt_set.set("motor".into()),
            "valve_analog" => fpt_set.set("valve_analog".into()),
            "valve" => fpt_set.set("valve".into()),
            _ => (),
        },
    };

    let close_fpt = move || fpt_set.set("".into());

    view! {
        <SvgDynamic
            file=svg_file
            svg_input=svg_input
            svg_output=svg_output
        />

        <drives::Motor
            title = "Двигатель"
            hmi_status = motor_status
            hmi_command = motor_command_set
            visible = Signal::derive(move || fpt.get() == "motor")
            on_close = close_fpt
        />

        <drives::ValveAnalog
            title = "Клапан аналоговый"
            hmi_status = valve_analog_status
            hmi_command = valve_analog_command
            visible = Signal::derive(move || fpt.get() == "valve_analog")
            on_close = close_fpt
        />

        <drives::Valve
            title = "Клапан"
            hmi_status = valve_status
            hmi_command = valve_command
            visible = Signal::derive(move || fpt.get() == "valve")
            on_close = close_fpt
        />

    }
}
