//! Dioxus-Bootstrap
//! YuKun Liu <mrxzx.info@gmai.com>
//!
//! Script Manager
//!  

use dioxus::core::Scope;
use golde::exec;

use crate::style::PresetColor;

pub struct Script {}

impl Script {
    /// create a alert in page
    pub fn alert<'a>(cx: &'a Scope, content: &'a str, style: PresetColor, target: &'a str) -> () {
        let cmd = format!(
            "dioxus.alert('{}', '{}', '{}')",
            target,
            content,
            style.to_string()
        );
        log::info!("{}", cmd);
        exec(&cx,cmd);
    }

    pub fn modal<'a>(cx: &'a Scope, id: &'a str) -> () {
        let cmd = format!(
            "window.dioxus_modal['{}'].show();",
            id
        );
        log::info!("{}", cmd);
        exec(&cx, cmd);
    }

}
