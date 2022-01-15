use dioxus::prelude::*;
use dioxus_bootstrap::prelude::*;
use golde::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {

    init_app(&cx);

    cx.render(rsx!{
        Application {
            Navbar {
                title: "Dioxus Bootstrap",
                container: true,
                background_color: PresetColor::Primary,
                NavItem {
                    text: "Repository",
                    href: "https://github.com/mrxiaozhuox/dioxus-bootstrap"
                }
            }
            br {}
            Container {
                Card {
                    border_color: PresetColor::Primary,
                    header: cx.render(rsx!(
                        h5 { "Dioxus" }
                    )),
                    CardTitle { text: "Dioxus Framework" },
                    CardText { 
                        text: "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." 
                    },
                    Link {
                        background_color: PresetColor::Primary,
                        text: "Website",
                        href: "https://dioxuslabs.com/",
                        target: "_blank",
                    }
                }
                br {}
                Card {
                    div {
                        id: "alert-block",
                        Alert {
                            alert_style: PresetColor::Primary,
                            close_button: true,
                            "Dioxus Bootstrap - Alert"
                        }
                    }
                    Button {
                        background_color: PresetColor::Warning,
                        text: "Show live alert",
                        onclick: move |_| {
                            execute(&cx, EXECUTE_TARGET, "dioxus.alert('#alert-block', 'Hello Dioxus-Bootstrap!', 'success')".into());
                        },
                    }
                }
                br {}
                Card {
                    div {
                        class: "dropdown",
                        button {
                            r#type: "button",
                            class: "btn btn-danger dropdown-toggle",
                            "data-bs-toggle": "dropdown",
                            "aria-expanded": "false",
                            "DropDown"
                        }
                        Dropdown {
                            items: vec![
                                DropdownItem::Link(DropdownItemLink::new("golde", "#")),
                                DropdownItem::Link(DropdownItemLink::new("bootstrap", "#")),
                                DropdownItem::Link(DropdownItemLink::new("tokio", "#")),
                                DropdownItem::Divider,
                                DropdownItem::Link(DropdownItemLink::new("dioxus", "#")),
                            ]
                        }
                    }
                }
            }
        }
    })
}