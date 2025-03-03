use crate::{components::Dashboard, views::Home, Route};
use dioxus::prelude::*;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        Dashboard { prefix: id.to_string() }
    }
}
