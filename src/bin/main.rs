use dioxus::prelude::*;
use dioxus_bootstrap::prelude::*;
use golde::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    
    init_app(&cx, |_initialized| {

    });

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
                NavDropdown {
                    text: "Dropdown",
                    Dropdown {
                        items: vec![
                            DropdownItem::Link(DropdownItemLink::new("dioxus", "#"))
                        ]
                    }
                }
            }
            br {}
            Container {
                Card {
                    border_color: PresetColor::Primary,
                    header: cx.render(rsx!(
                        h5 { "Dioxus Bootstrap" }
                    )),
                    CardTitle { text: "Dioxus Framework" },
                    Badge {
                        background_color: PresetColor::Success
                        text: "YuKun Liu <mrxzx.info@gmail.com>"
                    }
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
                        text: "create a alert",
                        onclick: move |_| {
                            Script::alert(&cx,"Hello Dioxus-Bootstrap!", PresetColor::Success, "#alert-block");
                        },
                    }
                    span { " " }
                    Button {
                        border_color: PresetColor::Primary,
                        text: "create a alert",
                        onclick: move |_| {
                            Script::alert(&cx,"Hello Dioxus-Bootstrap!", PresetColor::Danger, "#alert-block");
                        },
                    }
                }
                br {}
                Card {
                    div {
                        class: "row row-cols-auto",
                        Column {
                            DropdownTrigger {
                                button_color: PresetColor::Primary,
                                text: "dropdown",
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
                        Column {
                            DropdownTrigger {
                                button_color: PresetColor::Dark,
                                text: "dark dropdown",
                                Dropdown {
                                    is_dark: true,
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
                br {}
                Card {
                    div {
                        Button {
                            text: "Open Modal",
                            onclick: move |_| {
                                Script::modal(&cx, "testModal");
                            }
                        }
                    }
                }
            }
            Modal {
                id: "testModal",
                static_backdrop: false,
                ModalHeader {
                    h5 {
                        class: "modal-title",
                        "Dioxus Modal",
                    }
                }
                ModalBody {
                    p { "Hello Wrold" }
                }
            }
        }
    })
}
