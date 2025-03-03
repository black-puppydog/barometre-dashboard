use crate::{components::Dashboard, views::Home, Route};
use dioxus::prelude::*;

const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS}
        Dashboard { prefix: id.to_string() }
    }
}
