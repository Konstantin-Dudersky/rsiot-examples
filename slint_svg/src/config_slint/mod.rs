use crate::messages::Msg;
use rsiot::{components::cmp_slint::*, executor::Component};
use slint::{ComponentHandle, Image, Weak};
use std::time::Duration;

slint::include_modules!();

const IMAGE: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg version = "1.1"
     baseProfile="full"
     xmlns = "http://www.w3.org/2000/svg"
     xmlns:xlink = "http://www.w3.org/1999/xlink"
     xmlns:ev = "http://www.w3.org/2001/xml-events"
     height = "400px"  width = "400px">
     <rect x="0" y="0" width="400" height="400"
          fill="none" stroke="black" stroke-width="5px" stroke-opacity="0.5"/>
     <g fill-opacity="0.6" stroke="black" stroke-width="0.5px">
        <circle cx="200px" cy="200px" r="104px" fill="red"   transform="translate(  0,-52)" />
        <circle cx="200px" cy="200px" r="104px" fill="blue"  transform="translate( 60, 52)" />
        <circle cx="200px" cy="200px" r="104px" fill="green" transform="translate(-60, 52)" />
     </g>
</svg>
"#;

pub fn cmp(slint_window: Weak<MainWindow>) -> Component<Config<Msg, MainWindow>, Msg> {
    let config = Config {
        slint_window,
        fn_input: |msg, w| {
            let input = w.global::<Input>();

            match msg {
                Msg::Counter(_) => (),
                Msg::ButtonPressed => (),
                Msg::SvgImage(image) => {
                    let im = Image::load_from_svg_data(&image).unwrap();
                    input.set_svg_image(im);
                }
            }
        },
        fn_output: |w, sender| {
            let output = w.global::<Output>();
            let sender_clone = sender.clone();
            output.on_button_pressed(move || {
                let msg = Msg::ButtonPressed;
                sender_clone.send(msg);
            });
        },
        output_period: Duration::from_millis(100),
    };
    Cmp::new(config)
}
