use rsiot::{components::cmp_svg::*, executor::Component};

use crate::messages::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        file: include_str!("../../input.svg"),
        fn_input: |msg| match msg {
            Msg::Counter(v) => {
                vec![
                    SvgChange {
                        id: "rect1",
                        change: vec![
                            SvgChangeType::ChangeAttr {
                                attr_name: "x",
                                new_value: "0".to_string(),
                            },
                            SvgChangeType::ChangeAttr {
                                attr_name: "y",
                                new_value: "0".to_string(),
                            },
                            SvgChangeType::ChangeAttrStyle {
                                attr_style_name: "fill",
                                new_value: "#FF0000".to_string(),
                            },
                        ],
                        change_childs: None,
                    },
                    SvgChange {
                        id: "text1",
                        change: vec![
                            SvgChangeType::ChangeAttr {
                                attr_name: "x",
                                new_value: "2".to_string(),
                            },
                            SvgChangeType::ChangeAttrStyle {
                                attr_style_name: "fill",
                                new_value: "#FFFFFF".to_string(),
                            },
                            SvgChangeType::ChangeText {
                                text: v.to_string(),
                            },
                        ],
                        change_childs: None,
                    },
                ]
            }
            _ => vec![],
        },
        fn_output: |image| Msg::SvgImage(image.to_vec()),
    };

    Cmp::new(config)
}
