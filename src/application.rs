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

pub fn Application<'a>(cx: Scope<'a, ApplicationProps<'a>>) -> Element {

    let triggers = trigger!(
        bootstrap_modal => |_, _| { /**/ }
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
            script {[include_str!("./assets/bootstrap.min.js")]}
        }
    ))
}