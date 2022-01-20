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
    let mut dialog_class = "modal-dialog modal-dialog-scrollable".to_string();
    if cx.props.centered {
        dialog_class = format!("{} modal-dialog-centered", dialog_class);
    }

    golde::use_once(&cx, || {
        golde::exec(
            &cx,
            format!(
                "if (typeof window.dioxus_modal !== Object) {{ window.dioxus_modal = {{}}; }};
                window.dioxus_modal['{}'] = new bootstrap.Modal(document.getElementById('{}'), {{}});", 
                cx.props.id,
                cx.props.id,
            )
        );
    });

    if cx.props.static_backdrop {
        return cx.render(rsx!(
            div {
                class: "modal fade",
                id: "{cx.props.id}",
                "data-bs-backdrop": "static",
                "data-bs-keyboard": "{cx.props.static_backdrop}",
                "tabindex": "-1",
                "aria-hidden": "true",

                div {
                    class: "{dialog_class}",
                    div {
                        class: "modal-content",
                        &cx.props.children,
                    }
                }
            }
        ));
    } else {
        return cx.render(rsx!(
            div {
                class: "modal fade",
                id: "{cx.props.id}",
                "data-bs-keyboard": "{cx.props.static_backdrop}",
                "tabindex": "-1",
                "aria-hidden": "true",

                div {
                    class: "{dialog_class}",
                    div {
                        class: "modal-content",
                        &cx.props.children,
                    }
                }
            }
        ));
    };
}

#[inline_props]
pub fn ModalBody<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    cx.render(rsx!(
        div {
            class: "modal-body",
            &*children,
        }
    ))
}

#[inline_props]
pub fn ModalHeader<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    cx.render(rsx!(
        div {
            class: "modal-header",
            &*children,
        }
    ))
}

#[inline_props]
pub fn ModalFooter<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    cx.render(rsx!(
        div {
            class: "modal-footer",
            &*children,
        }
    ))
}
