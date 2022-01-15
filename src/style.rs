//! Dioxus-Bootstrap
//! YuKun Liu <mrxzx.info@gmai.com>
//! 
//! Style Enum
//!  

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PresetColor {

    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,

    Default,

    // link just can use in buttons
    Link
}

impl ToString for PresetColor {
    fn to_string(&self) -> String {
        let str = match &self {
            PresetColor::Primary => "primary",
            PresetColor::Secondary => "secondary",
            PresetColor::Success => "sucess",
            PresetColor::Danger => "danger",
            PresetColor::Warning => "warning",
            PresetColor::Info => "info",
            PresetColor::Light => "light",
            PresetColor::Dark => "dark",
            PresetColor::Default => "",
            PresetColor::Link => "link",
        };
        str.to_string()
    }
}

impl Default for PresetColor {
    fn default() -> Self { PresetColor::Default }
}

impl PresetColor {

    pub fn text_light(&self) -> bool {
        match &self {
            PresetColor::Primary => true,
            PresetColor::Secondary => true,
            PresetColor::Success => true,
            PresetColor::Danger => true,
            PresetColor::Warning => false,
            PresetColor::Info => false,
            PresetColor::Light => false,
            PresetColor::Dark => true,
            PresetColor::Default => false,
            PresetColor::Link => false,
        }
    }

}