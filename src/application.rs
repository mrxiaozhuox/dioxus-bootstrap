//! Dioxus-Bootstrap
//! YuKun Liu <mrxzx.info@gmai.com>
//!
//! Component Applcation
//!  
//! the basement component for `dioxus-boostrap`
//!

use dioxus::prelude::*;
use golde::*;

#[allow(dead_code)]
#[derive(Props)]
pub struct ApplicationProps<'a> {
    #[props(default)]
    trigger: Trigger,

    children: Element<'a>,
}

pub const EXECUTE_TARGET: &'static str = "_dioxus_custom_event";

pub fn Application<'a>(cx: Scope<'a, ApplicationProps<'a>>) -> Element {
    let initialized = cx.use_hook(|_| false);
    exec_conditional(&cx, "window.dioxus_modal = {};".into(), !*initialized);
    if !*initialized {
        log::info!("INIT!!!");

        // call_once code.
        *initialized = true;
    }

    let triggers = trigger!(
        bootstrap_modal => |_, _| { /**/ },
        _dioxus_custom_event => |code, res| {
            log::info!("Code: {} Result: {:?}", code, res);
        }
    );

    cx.render(rsx!(
        golde::App {
            trigger: triggers,
            style { [include_str!("./assets/bootstrap.min.css")] }
            div {
                id: "dioxus-bootstrap-main",
                &cx.props.children,
            }
            script {[include_str!("./assets/jquery.min.js")]}
            script {[include_str!("./assets/bootstrap.bundle.min.js")]}
            script {[include_str!("./assets/dioxus.event.js")]}
        }
    ))
}
