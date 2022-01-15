//! Dioxus-Bootstrap
//! YuKun Liu <mrxzx.info@gmai.com>
//! 
//! Component Navbar
//! 

use dioxus::prelude::*;

#[derive(Props)]
pub struct DropdownProps<'a> {
    items: Vec<DropdownItem<'a>>,
}

pub fn Dropdown<'a>(cx: Scope<'a, DropdownProps<'a>>) -> Element {

    let item_list = cx.props.items.iter().map(|val| {
        let v = match val {
            DropdownItem::Link(v) => {
                rsx!(
                    li {
                        a {
                            class: "dropdown-item",
                            href: "{v.href}",
                            "{v.text}"
                        }
                    }
                )
            },
            DropdownItem::Divider => {
                rsx!(
                    li {
                        hr {
                            class: "dropdown-divider"
                        }
                    }
                )
            }
        };
        return v;
    });

    cx.render(rsx!(
        ul {
            class: "dropdown-menu",
            item_list
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