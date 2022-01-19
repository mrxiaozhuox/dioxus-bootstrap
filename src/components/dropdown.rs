//! Dioxus-Bootstrap
//! YuKun Liu <mrxzx.info@gmai.com>
//!
//! Component Navbar
//!

use dioxus::prelude::*;

use crate::style::PresetColor;

#[derive(Props)]
pub struct DropdownProps<'a> {
    #[props(default)]
    is_dark: bool,

    items: Vec<DropdownItem<'a>>,
}

/// Component: Dropdown
///
/// PS: this component don't have any button or tool to trigger it,
/// if you want to trigger it, maybe you need use `DropdownTrigger` parcel this components.
///
pub fn Dropdown<'a>(cx: Scope<'a, DropdownProps<'a>>) -> Element {
    let mut index = 0;
    let item_list = cx.props.items.iter().map(|val| {
        let v = match val {
            DropdownItem::Link(v) => {
                let md5_key = md5::compute(format!("@link.{}.{}", v.text, v.href).as_bytes());
                let md5_key = format!("{:x}", md5_key);
                rsx!(
                    li {
                        key: "{md5_key}",
                        a {
                            class: "dropdown-item",
                            href: "{v.href}",
                            "{v.text}"
                        }
                    }
                )
            }
            DropdownItem::Divider => {
                let md5_key = md5::compute(format!("@divider.{}", index).as_bytes());
                let md5_key = format!("{:x}", md5_key);
                rsx!(
                    li {
                        key: "{md5_key}",
                        hr {
                            class: "dropdown-divider"
                        }
                    }
                )
            }
        };

        index += 1;

        return v;
    });

    let mut class_name = String::from("dropdown-menu");
    if cx.props.is_dark {
        class_name = String::from("dropdown-menu dropdown-menu-dark");
    }

    cx.render(rsx!(ul {
        class: "{class_name}",
        item_list
    }))
}

#[derive(Props)]
pub struct DropdownTriggerProps<'a> {
    #[props(default)]
    button_color: PresetColor,

    text: &'a str,

    children: Element<'a>,
}

pub fn DropdownTrigger<'a>(cx: Scope<'a, DropdownTriggerProps<'a>>) -> Element {
    let mut class_name = String::from("btn btn-primary dropdown-toggle");
    if cx.props.button_color != PresetColor::Default {
        class_name = format!(
            "btn btn-{} dropdown-toggle",
            cx.props.button_color.to_string()
        );
    }

    cx.render(rsx!(
        div {
            class: "dropdown",
            button {
                class: "{class_name}",
                r#type: "button",
                "data-bs-toggle": "dropdown",
                "aria-expanded": "false",
                "{cx.props.text}"
            }
            &cx.props.children,
        }
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropdownItem<'a> {
    Link(DropdownItemLink<'a>),
    Divider,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DropdownItemLink<'a> {
    href: &'a str,
    text: &'a str,
    target: &'a str,
}

impl<'a> DropdownItemLink<'a> {
    pub fn new(text: &'a str, href: &'a str) -> Self {
        Self {
            href,
            text,
            target: "_self",
        }
    }
}
