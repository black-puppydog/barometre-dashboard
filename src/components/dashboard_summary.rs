use dioxus::prelude::*;

use crate::types::CommunePropertiesSlim;

#[component]
pub fn DashboardSummary(progresses: Vec<CommunePropertiesSlim>) -> Element {
    rsx! {
        div{
            id: "dashboard",
            class: "w-full rounded-xl border-2 border-yellow-200 grid grid-cols-3",
            div{
                class: "text-center text-green-400 bg-gray-300 border-blue rounded m-5",
                "1. header"
            },
            div { class: "text-center text-green-400 rounded, border-green-300 border-2",
                "2. header"
            },
            div { class: "text-center text-bold text-red-400 rounded border-red-400 border-2",
                "3. header"
            }
        }
    }
}
