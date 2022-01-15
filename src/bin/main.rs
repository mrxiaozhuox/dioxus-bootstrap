use dioxus::prelude::*;
use dioxus_bootstrap::prelude::*;
use golde::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {

    init_app(&cx);

    cx.render(rsx!{
        Application {
            br {}
            Container {
                Card {
                    header: cx.render(rsx!(
                        h5 { "Featured" }
                    )),
                    CardTitle { text: "Special title treatment" },
                    CardText { text: "With supporting text below as a natural lead-in to additional content." },
                    a {
                        href: "#",
                        class: "btn btn-primary",
                        "Hello World"
                    }
                }
            }
        }
    })
}