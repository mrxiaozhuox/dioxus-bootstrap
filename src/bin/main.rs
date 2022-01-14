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
            Container {
                // div { style: "width:100%;height:100px;background-color:red;" }
            }
        }
    })
}