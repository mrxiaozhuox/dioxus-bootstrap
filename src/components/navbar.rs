//! Dioxus-Bootstrap
//! YuKun Liu <mrxzx.info@gmai.com>
//! 
//! Component Navbar
//!  

use dioxus::prelude::*;

use crate::style::PresetColor;


#[derive(Props)]
pub struct NavbarProps<'a> {
    
    #[props(default)]
    background_color: PresetColor,

    #[props(default)]
    container: bool,

    title: &'a str,

    children: Element<'a>,

}

pub fn Navbar<'a>(cx: Scope<'a, NavbarProps<'a>>) -> Element {
    
    let mut class_name = String::from("navbar navbar-light bg-light");
    if cx.props.background_color != PresetColor::Default {
        let text = if !cx.props.background_color.text_light() { "light" } else { "dark" };
        class_name = format!("navbar navbar-{} bg-{}", text, cx.props.background_color.to_string());
    }
    
    let container_name = if cx.props.container { "container" } else { "container-fluid" };

    cx.render(rsx!(
        div {
            class: "{class_name} navbar-expand-lg",
            div {
                class: "{container_name}",
                a {
                    class: "navbar-brand",
                    href: "#",
                    "{cx.props.title}"
                }
                button {
                    class: "navbar-toggler",
                    r#type: "button",
                    "navbar-toggler": "collapse",
                    "data-bs-target": "#navbar-navs",
                    "aria-expanded": "false",
                    span {
                        class: "navbar-toggler-icon",
                    }
                }
                div {
                    class: "collapse navbar-collapse",
                    id: "navbar-navs",
                    ul {
                        class: "navbar-nav",
                        &cx.props.children
                    }
                }
            }
        }
    ))
}

#[derive(Props)]
pub struct NavItemProps<'a> {

    #[props(default)]
    active: bool,

    #[props(default)]
    disabled: bool,

    #[props(default)]
    href: &'a str,

    #[props(default)]
    native: Element<'a>,

    text: &'a str

}

pub fn NavItem<'a>(cx: Scope<'a, NavItemProps<'a>>) -> Element {
    
    if cx.props.native.is_some() {
        return cx.render(rsx!(
            li {
                class: "nav-item",
                &cx.props.native,
            }
        ));
    }
    
    let href = if cx.props.href == "" { "#" } else { cx.props.href };
    let mut sub_class_name = String::from("nav-link");
    if cx.props.active {
        sub_class_name = String::from("nav-link active");
    } else if cx.props.disabled {
        sub_class_name = String::from("nav-link disabled");
    }

    cx.render(rsx!(
        li {
            class: "nav-item",
            a {
                class: "{sub_class_name}",
                href: "{href}",
                "{cx.props.text}"
            }
        }
    ))

}