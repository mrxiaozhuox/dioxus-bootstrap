//! Dioxus-Bootstrap
//! YuKun Liu <mrxzx.info@gmai.com>
//!
//! Component Button
//!  

use dioxus::{events::MouseEvent, prelude::*};

use crate::{prelude::Size, style::PresetColor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonType {
    Submit,
    Reset,
    Button,
}

impl ToString for ButtonType {
    fn to_string(&self) -> String {
        match &self {
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
            ButtonType::Button => "button",
        }
        .to_string()
    }
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::Button
    }
}

#[derive(Props)]
pub struct ButtonProps<'a> {
    #[props(default)]
    button_type: ButtonType,

    #[props(default)]
    background_color: PresetColor,

    #[props(default)]
    border_color: PresetColor,

    #[props(default)]
    size: Size,

    #[props(default)]
    disabled: bool,

    onclick: EventHandler<'a, MouseEvent>,

    text: &'a str,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let button_type = cx.props.button_type.to_string();

    let mut class_name = String::from("btn");
    if cx.props.background_color != PresetColor::Default {
        class_name = format!("btn btn-{}", cx.props.background_color.to_string());
    } else if cx.props.border_color != PresetColor::Default {
        class_name = format!("btn btn-outline-{}", cx.props.border_color.to_string());
    }

    if cx.props.size != Size::Default {
        class_name += &("btn".to_owned() + &cx.props.size.to_string());
    }

    cx.render(rsx!(
        button {
            r#type: "{button_type}",
            class: "{class_name}",
            disabled: "{cx.props.disabled}",
            onclick: move |evt| {
                cx.props.onclick.call(evt);
            },
            "{cx.props.text}"
        }
    ))
}

#[derive(Props)]
pub struct LinkProps<'a> {
    #[props(default)]
    background_color: PresetColor,

    #[props(default)]
    border_color: PresetColor,

    #[props(default)]
    size: Size,

    #[props(default)]
    href: &'a str,

    #[props(default)]
    target: &'a str,

    text: &'a str,
}

pub fn Link<'a>(cx: Scope<'a, LinkProps<'a>>) -> Element {
    let mut class_name = String::from("btn");
    if cx.props.background_color != PresetColor::Default {
        class_name = format!("btn btn-{}", cx.props.background_color.to_string());
    } else if cx.props.border_color != PresetColor::Default {
        class_name = format!("btn btn-outline-{}", cx.props.border_color.to_string());
    }

    if cx.props.size != Size::Default {
        class_name += &("btn".to_owned() + &cx.props.size.to_string());
    }

    let href = if cx.props.href == "" {
        "#"
    } else {
        cx.props.href
    };
    let target = if cx.props.target == "" {
        "_self"
    } else {
        cx.props.target
    };

    cx.render(rsx!(
        a {
            class: "{class_name}",
            href: "{href}",
            target: "{target}",
            "{cx.props.text}"
        }
    ))
}

#[derive(Props)]
pub struct ButtonGroupProps<'a> {
    #[props(default)]
    toolbar: bool,

    #[props(default)]
    vertical: bool,

    #[props(default)]
    style: &'a str,

    children: Element<'a>,
}

pub fn ButtonGroup<'a>(cx: Scope<'a, ButtonGroupProps<'a>>) -> Element {
    let mut class_name = String::from("btn-group");

    if cx.props.toolbar {
        class_name = String::from("btn-toolbar");
    } else if cx.props.vertical {
        class_name = String::from("btn-group-vertical");
    }

    cx.render(rsx!(
        div {
            class: "{class_name}",
            style: "{cx.props.style}",
            role: "group",
            &cx.props.children,
        }
    ))
}
