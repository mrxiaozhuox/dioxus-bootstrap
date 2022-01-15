//! Dioxus-Bootstrap
//! YuKun Liu <mrxzx.info@gmai.com>
//! 
//! Component Alert
//!  

use dioxus::prelude::*;

use crate::style::PresetColor;

#[derive(Props)]
pub struct AlertProps<'a> {
    
    #[props(default)]
    alert_style: PresetColor,

    #[props(default)]
    close_button: bool,

    children: Element<'a>
}

/// Bootstrap Component: Alert
/// 
/// Props:
/// - alert_style \[:optional\] the alert style <type: style::PresetColor>
/// - close_button \[:optional\] the close button in alert <type: boolean>
/// 
pub fn Alert<'a>(cx: Scope<'a, AlertProps<'a>>) -> Element {
    
    let class_name;
    if cx.props.alert_style == PresetColor::Default {
        class_name = String::from("alert alert-primary");
    } else {
        class_name = format!("alert alert-{}", cx.props.alert_style.to_string());
    }
    
    if cx.props.close_button {
        return cx.render(rsx!(
            div {
                class: "{class_name} alert-dismissible fade show",
                role: "alert",
                span {
                    &cx.props.children
                }
                button {
                    r#type: "button",
                    class: "btn-close",
                    "data-bs-dismiss": "alert",
                }
            }
        ));
    } else {
        return cx.render(rsx!(
            div {
                class: "{class_name}",
                role: "alert",
                &cx.props.children
            }
        ));
    }
}