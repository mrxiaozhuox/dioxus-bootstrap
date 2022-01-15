//! Dioxus-Bootstrap
//! YuKun Liu <mrxzx.info@gmai.com>
//! 
//! Component Card
//!  

use dioxus::prelude::*;

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

    children: Element<'a>

}

pub fn Card<'a>(cx: Scope<'a, CardProps<'a>>) -> Element {
    
    // if the `native` is set, return the native card code
    if cx.props.native.is_some() {
        return cx.render(rsx!(
            div {
                class: "card",
                style: "{cx.props.style}",
                &cx.props.native
            }
        ));
    }

    let class_name = String::from("card");
    
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