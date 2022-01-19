//! Dioxus-Bootstrap
//! YuKun Liu <mrxzx.info@gmai.com>
//!
//! Component Layout
//!  

use dioxus::prelude::*;

/// Dioxus-Bootstrap Layout::Size
///
/// use for change the layout (or component) size.
///
/// ```rust
/// fn App(cx: Scope) {
///     cx.render(rsx!(
///         Application {
///             Containier {
///                 size: Size::MD
///                 p { "Hello World" }
///             }
///         }
///     ))
/// }
/// ```
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    SM,
    MD,
    LG,
    XL,
    XXL,

    Fluid,
    Default,
}

impl Size {
    pub fn to_string(self) -> String {
        match self {
            Size::SM => "-sm".into(),
            Size::MD => "-md".into(),
            Size::LG => "-lg".into(),
            Size::XL => "-xl".into(),
            Size::XXL => "-xxl".into(),
            Size::Fluid => "-fluid".into(),
            Size::Default => "".into(),
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Props)]
pub struct ContainerProps<'a> {
    #[props(default)]
    size: Size,

    children: Element<'a>,
}

pub fn Container<'a>(cx: Scope<'a, ContainerProps<'a>>) -> Element {
    let class_name = format!("container{}", cx.props.size.to_string());

    cx.render(rsx!(
        div {
            class: "{class_name}",
            &cx.props.children
        }
    ))
}

#[derive(Props)]
pub struct GridProps<'a> {
    #[props(default)]
    row_max_cols: u8,

    children: Element<'a>,
}

pub fn Grid<'a>(cx: Scope<'a, GridProps<'a>>) -> Element {
    let mut class_name = String::from("row");

    if cx.props.row_max_cols != 0 {
        class_name = format!("row row-cols-{}", cx.props.row_max_cols);
    }

    cx.render(rsx!(
        div {
            class: "{class_name}",
            &cx.props.children
        }
    ))
}

#[derive(Props)]
pub struct ColumnProps<'a> {
    #[props(default)]
    size: Size,

    #[props(default)]
    length: u8,

    children: Element<'a>,
}

pub fn Column<'a>(cx: Scope<'a, ColumnProps<'a>>) -> Element {
    let mut class_name = String::from("col");

    if cx.props.size == Size::Default && cx.props.length != 0 {
        class_name = format!("col-{}", cx.props.length);
    } else if cx.props.size != Size::Default && cx.props.length != 0 {
        class_name = format!("col col-{}-{}", cx.props.size.to_string(), cx.props.length);
    }

    cx.render(rsx!(
        div {
            class: "{class_name}",
            &cx.props.children,
        }
    ))
}
