use dioxus::prelude::*;

mod components;
use components::Dashboard;
mod types;
mod views;
use views::Home;

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {"Not found"}
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/?:prefix")]
    Dashboard { prefix: String },
    // #[route("/:..segments")]
    // NotFound{segments: Vec<String>},
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "icon", type: "image/png", href: asset!("/assets/favicon.png") }
        Router::<Route> {}
    }
}
