//! Dioxus-Bootstrap
//! YuKun Liu <mrxzx.info@gmai.com>
//!
//! Component Modal
//!

use dioxus::prelude::*;

#[derive(Props)]
pub struct ModalProps<'a> {
    id: &'a str,

    #[props(default)]
    static_backdrop: bool,

    #[props(default)]
    centered: bool,

    children: Element<'a>,
}

pub fn Modal<'a>(cx: Scope<'a, ModalProps<'a>>) -> Element {
    let backdrop = if cx.props.static_backdrop {
        "static"
    } else {
        "dynamic"
    };

    let mut dialog_class = "modal-dialog modal-dialog-scrollable".to_string();
    if cx.props.centered {
        dialog_class = format!("{} modal-dialog-centered", dialog_class);
    }

    cx.render(rsx!(
        div {
            class: "modal fade",
            id: "{cx.props.id}",
            "data-bs-backdrop": "{backdrop}",
            "data-bs-keyboard": "{cx.props.static_backdrop}",
            "tabindex": "-1",
            "aria-hidden": "true",

            div {
                class: "{dialog_class}",
                div {
                    class: "model-content",
                }
            }

        }
    ))
}
