//! Dioxus-Bootstrap
//! YuKun Liu <mrxzx.info@gmai.com>
//!
//! Script Manager
//!  

use dioxus::core::{Scope, ScopeState};
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
        exec(&cx, cmd);
    }

    pub fn modal<'a>(cx: &'a Scope, id: &'a str) -> ModalScript<'a> {
        // exec(&cx, cmd);

        ModalScript { cx, id }
    }
}

pub struct ModalScript<'a> {
    cx: &'a ScopeState,
    id: &'a str,
}

impl<'a> ModalScript<'a> {
    pub fn show(&mut self) {
        let cmd = format!("window.dioxus_modal['{}'].show();", self.id);
        exec(self.cx, cmd);
    }

    pub fn hide(&mut self) {
        let cmd = format!("window.dioxus_modal['{}'].hide();", self.id);
        exec(self.cx, cmd);
    }
}
