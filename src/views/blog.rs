use crate::{components::Dashboard, views::Home, Route};
use dioxus::prelude::*;

#[component]
pub fn Blog(prefix: String) -> Element {
    rsx! {
        Dashboard { prefix }
    }
}
