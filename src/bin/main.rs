use dioxus::prelude::*;
use dioxus_bootstrap::prelude::*;
use golde::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {

    wasm_logger::init(wasm_logger::Config::default());

    init_app(&cx);

    cx.render(rsx!{
        Application {
            br {}
            Container {
                Card {
                    border_color: PresetColor::Primary,
                    header: cx.render(rsx!(
                        h5 { "Featured" }
                    )),
                    CardTitle { text: "Special title treatment" },
                    CardText { text: "With supporting text below as a natural lead-in to additional content." },
                    Link {
                        background_color: PresetColor::Primary,
                        text: "123",
                        
                    }
                }
            }
        }
    })
}