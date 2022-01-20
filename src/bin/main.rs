use dioxus::prelude::*;
use dioxus_bootstrap::{prelude::*, components::progress::Progress};
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
                    h5 { "Alerts" }
                    hr {}
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
                    h5 { "Dropdown Tool" }
                    hr {}
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
                    h5 { "Modal Dsiplay" }
                    hr {}
                    div {
                        Button {
                            text: "Open Modal",
                            onclick: move |_| {
                                let mut my_modal = Script::modal(&cx, "testModal");
                                my_modal.show();
                            }
                        }
                    }
                }
                br {}
                Card {
                    h5 { "Progress" }
                    hr {}
                    p {
                        Progress {
                            percentage: 25
                        }
                    }
                    p {
                        Progress {
                            percentage: 50,
                            text: "50%".to_string(),
                            color: PresetColor::Success,
                        }
                    }
                    p {
                        Progress {
                            percentage: 75,
                            text: "75%".to_string(),
                            color: PresetColor::Info,
                            striped: true,
                        }
                    }
                    p {
                        Progress {
                            percentage: 100,
                            text: "100%".to_string(),
                            color: PresetColor::Dark,
                            striped: true,
                            animated: true,
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
