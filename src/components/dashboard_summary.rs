use dioxus::prelude::*;

use crate::types::CommunePropertiesSlim;

#[component]
pub fn DashboardSummary(progresses: Vec<CommunePropertiesSlim>) -> Element {
    let qualified = progresses.iter().filter(|c| c.progress() >= 100f32).count();
    let close = progresses
        .iter()
        .filter(|c| c.target_contributions() - c.contributions as usize <= 10)
        .count();
    rsx! {
        div{
            class: "w-3/4 rounded-xl grid grid-cols-3 m-auto my-5",
            div{
                class: "text-center border-2 border-gray-500 rounded-2xl m-5 p-5 text-xl font-semibold",
                "{progresses.len()} communes"
            },
            div {
                class: "text-center border-2 border-gray-500 rounded-2xl m-5 p-5 text-xl font-semibold",
                "{qualified}",
                br{},
                "qualifiées",
            },
            div {
                class: "text-center border-2 border-gray-500 rounded-2xl m-5 p-5 text-xl font-semibold",
                "{close}",
                br{},
                "presque qualifiées"
            }
        }
    }
}
