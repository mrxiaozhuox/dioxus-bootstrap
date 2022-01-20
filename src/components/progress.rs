//! Dioxus-Bootstrap
//! YuKun Liu <mrxzx.info@gmai.com>
//!
//! Component Progress
//!

use dioxus::prelude::*;

use crate::style::PresetColor;

#[derive(Props, PartialEq)]
pub struct ProgressProps {
    percentage: usize,

    #[props(default)]
    text: String,

    #[props(default)]
    color: PresetColor,

    #[props(default)]
    striped: bool,

    #[props(default)]
    animated: bool,
}

pub fn Progress(cx: Scope<ProgressProps>) -> Element {
    let mut percentage = cx.props.percentage;
    while percentage > 100 {
        percentage -= 100;
    }

    let color = if cx.props.color == PresetColor::Default {
        "primary".into()
    } else {
        cx.props.color.to_string()
    };
    let mut bar_class = format!("progress-bar bg-{}", color);

    if cx.props.striped {
        bar_class = format!("{} progress-bar-striped", bar_class);
    }

    if cx.props.animated {
        bar_class = format!("{} progress-bar-animated", bar_class);
    }

    cx.render(rsx!(
        div {
            class: "progress",
            div {
                class: "{bar_class}",
                style: "width: {percentage}%",
                "{cx.props.text}",
            }
        }
    ))
}
