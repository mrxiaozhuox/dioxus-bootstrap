//! Dioxus-Bootstrap
//! YuKun Liu <mrxzx.info@gmai.com>
//!
//! Component Badge
//!  

use dioxus::prelude::*;

use crate::style::PresetColor;

#[derive(Props)]
pub struct BadgeProps<'a> {
    #[props(default)]
    background_color: PresetColor,

    #[props(default)]
    pill: bool,

    text: &'a str,
}

pub fn Badge<'a>(cx: Scope<'a, BadgeProps<'a>>) -> Element {
    let mut class_name = String::from("badge bg-secondary");
    if cx.props.background_color != PresetColor::Default {
        class_name = format!("badge bg-{}", cx.props.background_color.to_string());
    }
    if cx.props.pill {
        class_name = format!("{} rounded-pill", class_name);
    }

    cx.render(rsx!(
        span {
            class: "{class_name}",
            "{cx.props.text}"
        }
    ))
}
