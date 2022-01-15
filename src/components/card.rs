//! Dioxus-Bootstrap
//! YuKun Liu <mrxzx.info@gmai.com>
//! 
//! Component Card
//!  

use dioxus::prelude::*;

use crate::style::PresetColor;

#[derive(Props)]
pub struct CardProps<'a> {


    #[props(default)]
    header: Element<'a>,

    #[props(default)]
    style: &'a str,

    #[props(default)]
    footer: Element<'a>,

    #[props(default)]
    native: Element<'a>,

    #[props(default)]
    background_color: PresetColor,
    #[props(default)]
    border_color: PresetColor,

    children: Element<'a>

}

///
/// Bootstrap Component: Card
/// 
/// Props:
/// - header                \[:optional\] set card-header content
/// - fotter                \[:optional\] set card-footer content
/// - style                 \[:optional\] set card div style
/// - native                \[:optional\] custom all card internal structure
/// - background_color      \[:optional\] set card bg-style <type: style::PresetColor>
/// - border_color          \[:optional\] set card border-style <type: style::PresetColor>
/// 
/// 
pub fn Card<'a>(cx: Scope<'a, CardProps<'a>>) -> Element {
    
    let mut class_name = String::from("card");

    if cx.props.background_color != PresetColor::Default {

        let text_style = if cx.props.background_color.text_light() { "text-white" } else { "text-dark" };

        class_name = format!("card {} bg-{}", text_style, cx.props.background_color.to_string());

    } else if cx.props.border_color != PresetColor::Default {
        class_name = format!("card border-{}", cx.props.border_color.to_string());
    }

    // if the `native` is set, return the native card code
    if cx.props.native.is_some() {
        return cx.render(rsx!(
            div {
                class: "{class_name}",
                style: "{cx.props.style}",
                &cx.props.native
            }
        ));
    }
    
    let mut header = None;
    if cx.props.header.is_some() {
        header = cx.render(rsx!(
            div {
                class: "card-header",
                &cx.props.header
            }
        ));
    }

    let mut footer = None;
    if cx.props.footer.is_some() {
        footer = cx.render(rsx!(
            div {
                class: "card-footer",
                &cx.props.footer,
            }
        ));
    }

    cx.render(rsx!(
        div {
            class: "{class_name}",

            header,
            div {
                class: "card-body",
                style: "{cx.props.style}" ,
                
                div {
                    class: "card-body",
                    &cx.props.children
                }
            }
            footer,
        }
    ))   
}

#[inline_props]
pub fn CardTitle<'a>(cx: Scope<'a>, text: &'a str) -> Element {
    cx.render(rsx!(
        h5 {
            class: "card-title",
            "{text}"
        }
    ))
}

#[inline_props]
pub fn CardText<'a>(cx: Scope<'a>, text: &'a str) -> Element {
    cx.render(rsx!(
        p {
            class: "card-text",
            "{text}"
        }
    ))
}