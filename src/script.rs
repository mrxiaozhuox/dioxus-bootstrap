//! Dioxus-Bootstrap
//! YuKun Liu <mrxzx.info@gmai.com>
//!
//! Script Manager
//!  

use dioxus::core::Scope;
use golde::execute;

use crate::{prelude::EXECUTE_TARGET, style::PresetColor};

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
        execute(&cx, EXECUTE_TARGET, cmd);
    }
}
